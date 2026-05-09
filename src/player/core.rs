use std::sync::Arc;
use std::time::Duration;

use parking_lot::RwLock;
use rodio::{DeviceSinkBuilder, MixerDeviceSink, Player};
use tokio::sync::mpsc::UnboundedSender;

use crate::player::source::PlaySourceShared;
use crate::provider::TrackShared;

use super::errors::PlayerError;
use super::events::PlayCommand;
use super::queue::PendingIndexMap;
use super::state::{PlayState, PlayStateShared};
use super::types::PlayerShared;
use super::worker::Worker;

pub struct Playerith {
	player: PlayerShared,
	sink: MixerDeviceSink,

	state: PlayStateShared,

	sender: UnboundedSender<PlayCommand>,
	worker: Worker,
}

impl Playerith {
	pub fn init() -> PlayerError<Self> {
		let sink = DeviceSinkBuilder::open_default_sink()?;
		let player = Arc::new(Player::connect_new(sink.mixer()));
		let state = Arc::new(PlayState {
			queue: super::queue::PendingQueue::new(),
			curr: Arc::new(RwLock::new(None)),
		});

		let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();

		let worker = Worker::new(receiver, player.clone(), state.clone());

		Ok(Self {
			player,
			sink,

			state,

			sender,
			worker,
		})
	}
}

impl Playerith {
	pub fn seek(&self, delta: i64) {
		self.sender.send(PlayCommand::Seek(delta)).ok();
	}
	pub fn set_vol(&self, vol: f32) {
		self.sender
			.send(PlayCommand::Volume(vol.min(1.0).max(0.0)))
			.ok();
	}

	pub fn queue_snapshot(&self) -> PendingIndexMap {
		self.state.queue.snapshot()
	}

	pub fn current(&self) -> Option<TrackShared> {
		self.state.curr.read().clone()
	}

	pub fn current_pause(&self) {
		self.player.pause();
	}

	pub fn current_resume(&self) {
		self.player.play();
	}
	pub fn current_pos(&self) -> Duration {
		self.player.get_pos()
	}
	pub fn current_paused(&self) -> bool {
		self.player.is_paused()
	}
	pub fn current_vol(&self) -> f32 {
		self.player.volume()
	}

	pub fn skip_all(&self) {
		self.sender.send(PlayCommand::SkipAll).ok();
	}

	pub fn skip_current(&self) {
		self.sender.send(PlayCommand::SkipCurrent).ok();
	}

	pub fn skip(&self, track: &TrackShared) {
		self.sender.send(PlayCommand::Skip(track.clone())).ok();
	}

	pub fn shuffle(&self) {
		self.sender.send(PlayCommand::Shuffle).ok();
	}

	pub fn reorder(&self, from: usize, to: usize) {
		self.sender.send(PlayCommand::Reorder(from, to)).ok();
	}

	pub fn submit(&self, track: &TrackShared, source: PlaySourceShared) {
		self.sender
			.send(PlayCommand::Play(track.clone(), source))
			.ok();
	}
}
