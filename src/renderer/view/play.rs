use std::time::Duration;

use crate::provider::TrackShared;

pub struct PlayView<'a> {
	pub current: &'a Option<TrackShared>,
	pub pos: Duration,
	pub paused: bool,
	pub vol: f32,
}
