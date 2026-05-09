use crate::provider::TrackShared;

use super::source::PlaySourceShared;

pub enum PlayCommand {
	Skip(TrackShared),
	Play(TrackShared, PlaySourceShared),
	Reorder(usize, usize),
	Volume(f32),
	Seek(i64),

	Shuffle,
	SkipAll,
	SkipCurrent,
}
