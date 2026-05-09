use async_trait::async_trait;

use super::errors::ProviderError;
use super::track::TrackStreamMeta;
use super::types::{TrackList, TrackShared};

#[derive(Clone, Copy)]
pub enum ProviderType {
	SoundCloud,
}

impl ProviderType {
	pub fn name(&self) -> &'static str {
		match self {
			Self::SoundCloud => "SoundCloud",
		}
	}
}

pub struct SearchOptions {
	pub limit: u32,
}

#[async_trait]
pub trait Provider: Sync + Send {
	async fn search(
		&self,
		query: &str,
		opts: Option<&SearchOptions>,
	) -> ProviderError<TrackList>;
	async fn stream(
		&self,
		track: &TrackShared,
	) -> ProviderError<TrackStreamMeta>;
}
