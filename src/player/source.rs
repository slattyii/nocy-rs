use std::{pin::Pin, sync::Arc};

use crate::provider::{TrackShared, TrackStreamMeta};

use super::errors::SourceError;

pub type PlaySourceShared = Arc<PlaySource>;

pub enum PlaySource {
	StreamFetch(Fetcher),
}

// fetcher
pub type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;

pub struct Fetcher(
	Box<
		dyn Fn(&TrackShared) -> BoxFuture<SourceError<TrackStreamMeta>>
			+ Sync
			+ Send,
	>,
);

impl Fetcher {
	pub fn new<F, Fut>(f: F) -> Self
	where
		F: Fn(&TrackShared) -> Fut + Sync + Send + 'static,
		Fut: Future<Output = SourceError<TrackStreamMeta>> + Send + 'static,
	{
		Self(Box::new(move |t| Box::pin(f(t))))
	}

	pub async fn fetch_stream(
		&self,
		track: &TrackShared,
	) -> SourceError<TrackStreamMeta> {
		(self.0)(track).await
	}
}
