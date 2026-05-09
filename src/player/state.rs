use std::sync::Arc;

use parking_lot::RwLock;

use crate::provider::TrackShared;

use super::queue::PendingQueue;

pub type PlayStateShared = Arc<PlayState>;

pub struct PlayState {
	pub queue: PendingQueue,
	pub curr: Arc<RwLock<Option<TrackShared>>>,
}
