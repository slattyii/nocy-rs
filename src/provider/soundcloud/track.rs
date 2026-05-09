use std::{any::Any, time::Duration};

use super::Track;

#[derive(Debug)]
pub struct TranscodingFormat {
	pub mime_type: String,
	pub protocol: String,
}

#[derive(Debug)]
pub struct Transcoding {
	pub format: TranscodingFormat,
	pub url: String,
}

#[derive(Debug)]
pub struct SoundCloudTrack {
	pub id: String,
	pub title: String,
	pub artist: String,
	pub permalink_url: String,
	pub duration: Duration,
	pub created_at: String,
	pub track_authorization: Option<String>,
	pub transcodings: Option<Vec<Transcoding>>,
}

impl Track for SoundCloudTrack {
	fn as_any(&self) -> &dyn Any {
		self
	}
	fn provider(&self) -> crate::provider::ProviderType {
		crate::provider::ProviderType::SoundCloud
	}
	fn id(&self) -> &str {
		&self.id
	}
	fn url(&self) -> &str {
		&self.permalink_url
	}
	fn title(&self) -> &str {
		&self.title
	}
	fn artist(&self) -> &str {
		&self.artist
	}
	fn duration(&self) -> &Duration {
		&self.duration
	}
	fn stream(&self) -> Option<crate::provider::TrackStreamMeta> {
		None
	}
}
