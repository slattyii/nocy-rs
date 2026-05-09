use indexmap::IndexMap;
use parking_lot::RwLock;
use rand::seq::SliceRandom;
use std::sync::Arc;

use crate::provider::TrackShared;

use super::source::PlaySourceShared;

pub type PendingValue = (TrackShared, PlaySourceShared);
pub type PendingIndexMap = IndexMap<String, PendingValue>;

#[derive(Clone)]
pub struct PendingQueue {
	inner: Arc<RwLock<PendingIndexMap>>,
}

impl PendingQueue {
	pub fn new() -> Self {
		Self {
			inner: Arc::new(RwLock::new(IndexMap::new())),
		}
	}

	pub fn shuffle(&self) {
		let mut rng = rand::rng();
		let mut arr = self.inner.read().clone().into_iter().collect::<Vec<_>>();

		arr.shuffle(&mut rng);

		let shuffled = arr.into_iter().collect::<IndexMap<_, _>>();
		*self.inner.write() = shuffled;
	}

	pub fn clear(&self) {
		self.inner.write().clear();
	}

	pub fn push_front(&self, id: impl Into<String>, value: PendingValue) {
		let mut map = self.inner.write();
		map.shift_insert(0, id.into(), value);
	}

	pub fn push_back(&self, id: impl Into<String>, value: PendingValue) {
		self.push(id, value);
	}

	pub fn pop_front(&self) -> Option<(String, PendingValue)> {
		let mut map = self.inner.write();
		map.shift_remove_index(0).map(|(k, v)| (k, v))
	}

	pub fn pop_back(&self) -> Option<(String, PendingValue)> {
		let mut map = self.inner.write();
		let last = map.len().checked_sub(1)?;
		map.swap_remove_index(last)
	}

	pub fn push(&self, id: impl Into<String>, value: PendingValue) {
		self.inner.write().insert(id.into(), value);
	}

	pub fn remove(&self, id: &str) -> Option<PendingValue> {
		self.inner.write().shift_remove(id)
	}

	pub fn get(&self, id: &str) -> Option<PendingValue> {
		self.inner.read().get(id).cloned()
	}

	pub fn contains(&self, id: &str) -> bool {
		self.inner.read().contains_key(id)
	}

	pub fn swap_by_index(&self, index_a: usize, index_b: usize) -> bool {
		let mut map = self.inner.write();
		let len = map.len();
		if index_a < len && index_b < len {
			map.swap_indices(index_a, index_b);
			true
		} else {
			false
		}
	}

	pub fn swap(&self, id_a: &str, id_b: &str) -> bool {
		let mut map = self.inner.write();
		match (map.get_index_of(id_a), map.get_index_of(id_b)) {
			(Some(a), Some(b)) => {
				map.swap_indices(a, b);
				true
			}
			_ => false,
		}
	}

	pub fn with<F, R>(&self, id: &str, f: F) -> Option<R>
	where
		F: FnOnce(&PendingValue) -> R,
	{
		self.inner.read().get(id).map(f)
	}

	pub fn snapshot(&self) -> PendingIndexMap {
		self.inner.read().clone()
	}

	pub fn len(&self) -> usize {
		self.inner.read().len()
	}

	pub fn is_empty(&self) -> bool {
		self.inner.read().is_empty()
	}
}
