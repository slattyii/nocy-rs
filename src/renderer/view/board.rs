use crate::{player::PendingIndexMap, provider::TrackShared};

pub struct BoardView<'a> {
	pub queue: &'a PendingIndexMap,
	pub items: &'a [TrackShared],
	pub selected: usize,
}
