use std::{any::Any, time::Duration};

use super::provider::ProviderType;

pub struct TrackStreamMeta {
	pub url: String,
}

pub trait Track: Sync + Send {
	fn as_any(&self) -> &dyn Any;
	fn provider(&self) -> ProviderType;
	fn id(&self) -> &str;
	fn url(&self) -> &str;
	fn title(&self) -> &str;
	fn artist(&self) -> &str;
	fn duration(&self) -> &Duration;
	fn stream(&self) -> Option<TrackStreamMeta>;
}
