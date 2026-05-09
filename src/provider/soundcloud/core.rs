use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use reqwest::Client;

use crate::provider::SearchOptions;

use super::auth::Authorization;
use super::track::{SoundCloudTrack, Transcoding, TranscodingFormat};
use super::{Provider, ProviderError, TrackList, TrackShared, TrackStreamMeta};

pub struct SoundCloudProvider {
	client: Client,
	auth: Authorization,
}

#[async_trait]
impl Provider for SoundCloudProvider {
	async fn search(
		&self,
		query: &str,
		opts: Option<&SearchOptions>,
	) -> ProviderError<TrackList> {
		let ret = self.internal_search(query, opts).await?;
		Ok(ret)
	}

	async fn stream(
		&self,
		track: &TrackShared,
	) -> ProviderError<TrackStreamMeta> {
		let ret = self.internal_stream_fetch(track).await?;
		Ok(ret)
	}
}

// constructor
impl SoundCloudProvider {
	pub fn new() -> Self {
		let client = Client::builder()
			.default_headers(super::session::ss_headers())
			.build()
			.unwrap_or_default();
		let auth = Authorization::new();

		Self {
			client,
			auth,
		}
	}
}

// auth
impl SoundCloudProvider {
	async fn get_client_id(&self) -> ProviderError<String> {
		const MARKER: &'static str = ",client_id:\"";
		const TTL: i64 = 7 * 24 * 60 * 60 * 1000;

		let auth = self.auth.get_inner();

		{
			if let Some(v) = auth.read().as_ref() {
				if chrono::Utc::now().timestamp_millis() - v.update_ts < TTL {
					return Ok(v.client_id.clone());
				}
			};
		}

		let html = self
			.client
			.get("https://soundcloud.com/")
			.send()
			.await?
			.text()
			.await?;

		let urls = {
			let doc = scraper::Html::parse_document(&html);
			let selector = scraper::Selector::parse("script[crossorigin]")
				.map_err(|_| anyhow::anyhow!("selector parse error"))?;
			doc.select(&selector)
				.filter_map(|e| e.value().attr("src").map(String::from))
				.collect::<Vec<_>>()
		};
		for url in &urls {
			let Ok(text) = self.client.get(url).send().await?.text().await
			else {
				continue;
			};

			let Some(idx) = text.find(MARKER) else {
				continue;
			};

			let start = idx + MARKER.len();
			let Some(end) = text[start..].find('"') else {
				continue;
			};

			let client_id = &text[start..start + end];
			if client_id.is_empty() {
				continue;
			};

			let auth = auth.clone();
			let authdata = Arc::new(super::auth::AuthorizationData {
				client_id: client_id.into(),
				update_ts: chrono::Utc::now().timestamp_millis(),
			});
			*auth.write() = Some(authdata.clone());

			self.auth.store(&authdata).ok();

			return Ok(authdata.client_id.to_string());
		}

		Err(anyhow::anyhow!("failed to extract any client id"))
	}
}

impl SoundCloudProvider {
	async fn internal_search(
		&self,
		query: &str,
		opts: Option<&SearchOptions>,
	) -> ProviderError<TrackList> {
		let client_id = self.get_client_id().await?;
		let limit = opts.map(|o| o.limit).unwrap_or(30).to_string();

		let res = self
			.client
			.get("https://api-v2.soundcloud.com/search/tracks")
			.query(&[
				("q", query),
				("client_id", &client_id),
				("limit", &limit),
				("offset", "0"),
				("linked_partition", "1"),
				("app_locale", "vn"),
			])
			.send()
			.await?
			.json::<serde_json::Value>()
			.await?;

		let collection = match res["collection"].as_array() {
			Some(c) => c,
			None => {
				return Err(anyhow::anyhow!(
					"no collection in search response"
				));
			}
		};

		let mut tracks = Vec::with_capacity(collection.len());

		for t in collection {
			const UNKNOWN: &str = "Unknown";

			let title = t["title"].as_str().unwrap_or(UNKNOWN);

			let id = match t["id"].as_u64() {
				Some(n) => n.to_string(),
				None => t["id"].as_str().unwrap_or(title).to_string(),
			};

			let artist = t["user"]["username"].as_str().unwrap_or(UNKNOWN);
			let permalink = t["permalink_url"].as_str().unwrap_or("");
			let created_at = t["created_at"].as_str().unwrap_or(UNKNOWN);
			let track_auth =
				t["track_authorization"].as_str().map(str::to_string);

			let duration = {
				let ms = t["duration"].as_u64().unwrap_or(0);
				Duration::from_secs(if ms > 0 { ms / 1000 } else { 0 })
			};

			let transcodings = match t["media"]["transcodings"].as_array() {
				Some(arr) => {
					let mut ts = Vec::with_capacity(arr.len());
					for item in arr {
						ts.push(Transcoding {
							format: TranscodingFormat {
								protocol: item["format"]["protocol"]
									.as_str()
									.unwrap_or("")
									.to_string(),
								mime_type: item["format"]["mime_type"]
									.as_str()
									.unwrap_or("")
									.to_string(),
							},
							url: item["url"].as_str().unwrap_or("").to_string(),
						});
					}
					Some(ts)
				}
				None => None,
			};

			tracks.push(Arc::new(SoundCloudTrack {
				id,
				title: title.to_string(),
				artist: artist.to_string(),
				permalink_url: permalink.to_string(),
				duration,
				track_authorization: track_auth,
				created_at: created_at.to_string(),
				transcodings,
			}) as TrackShared);
		}

		Ok(tracks)
	}

	async fn internal_stream_fetch(
		&self,
		track: &TrackShared,
	) -> ProviderError<TrackStreamMeta> {
		let client_id = self.get_client_id().await?;

		let (stream_url, track_auth) = if let Some(t) =
			track.as_any().downcast_ref::<SoundCloudTrack>()
		{
			let url = t
				.transcodings
				.as_deref()
				.and_then(|arr| {
					arr.iter().find(|t| t.format.protocol == "progressive")
				})
				.map(|t| t.url.clone());

			(url, t.track_authorization.clone())
		} else {
			let info = self.fetch_track_info(track.url()).await?;

			let url = match info["media"]["transcodings"].as_array() {
				Some(arr) => arr
					.iter()
					.find(|t| {
						t["format"]["protocol"].as_str() == Some("progressive")
					})
					.and_then(|t| t["url"].as_str())
					.map(str::to_string),
				None => None,
			};

			(
				url,
				info["track_authorization"].as_str().map(str::to_string),
			)
		};

		let stream_url = stream_url.ok_or_else(|| {
			anyhow::anyhow!("no progressive stream url found")
		})?;
		let track_auth = track_auth
			.ok_or_else(|| anyhow::anyhow!("no track authorization found"))?;

		let url = self
			.client
			.get(&stream_url)
			.query(&[
				("client_id", &client_id),
				("track_authorization", &track_auth),
			])
			.send()
			.await?
			.json::<serde_json::Value>()
			.await?["url"]
			.as_str()
			.map(str::to_string)
			.ok_or_else(|| {
				anyhow::anyhow!("stream url not found in response")
			})?;

		Ok(TrackStreamMeta {
			url,
		})
	}

	async fn fetch_track_info(
		&self,
		url: &str,
	) -> ProviderError<serde_json::Value> {
		let client_id = self.get_client_id().await?;
		Ok(self
			.client
			.get("https://api-v2.soundcloud.com/resolve")
			.query(&[("url", url), ("client_id", &client_id)])
			.send()
			.await?
			.json::<serde_json::Value>()
			.await?)
	}
}

#[tokio::test]
async fn scl() {
	let provider = SoundCloudProvider::new();
	let tracks = provider.search("j97", None).await.unwrap();
	let mut current: Option<std::process::Child> = None;

	loop {
		let mut options = vec!["exit"];
		for t in &tracks {
			options.push(t.title());
		}

		match inquire::Select::new("Play:", options).prompt() {
			Ok(c) if c == "exit" => break,
			Ok(c) => {
				if let Some(mut c) = current.take() {
					c.kill().ok();
					c.wait().ok();
				}

				let track = tracks.iter().find(|t| t.title() == c).unwrap();
				let stream = provider.stream(&track).await.unwrap();

				match Command::new("mpv")
					.args(["--quiet", "--really-quiet", &stream.url])
					.stdin(std::process::Stdio::null())
					.stdout(std::process::Stdio::null())
					.stderr(std::process::Stdio::null())
					.spawn()
				{
					Ok(c) => {
						current = Some(c);
					}
					Err(e) => {
						eprintln!("{}", e.to_string());
					}
				}
			}
			Err(_) => break,
		};
	}

	if let Some(mut c) = current.take() {
		c.kill().ok();
		c.wait().ok();
	}
}
