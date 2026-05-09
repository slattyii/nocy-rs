use std::sync::Arc;

use super::track::Track;

pub type TrackShared = Arc<dyn Track>;
pub type TrackList = Vec<TrackShared>;
