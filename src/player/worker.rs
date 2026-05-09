use std::time::Duration;

use crate::{
	player::{Fetcher, PlaySource},
	provider::TrackShared,
};

use super::events::PlayCommand;
use super::state::PlayStateShared;
use super::types::PlayerShared;

use rodio::Decoder;
use tokio::sync::{
	mpsc::{self, Receiver, Sender, UnboundedReceiver},
	watch,
};

type EventReceiver = UnboundedReceiver<PlayCommand>;
type WakeSender = Sender<()>;
type WakeReceiver = Receiver<()>;

pub struct Worker {
	stop_tx: watch::Sender<bool>,
}

impl Worker {
	pub fn new(
		receiver: EventReceiver,
		player: PlayerShared,
		state: PlayStateShared,
	) -> Self {
		let (stop_tx, stop_rx) = watch::channel(false);
		let (wake_tx, wake_rx) = mpsc::channel::<()>(1);

		Self::spawn_event_loop(
			receiver,
			player.clone(),
			state.clone(),
			wake_tx,
			stop_rx.clone(),
		);
		Self::spawn_machine_loop(
			player.clone(),
			state.clone(),
			wake_rx,
			stop_rx.clone(),
		);

		Self {
			stop_tx,
		}
	}

	pub fn stop(&self) {
		self.stop_tx.send(true).ok();
	}
}

impl Drop for Worker {
	fn drop(&mut self) {
		self.stop();
	}
}

impl Worker {
	fn spawn_event_loop(
		mut receiver: EventReceiver,
		player: PlayerShared,
		state: PlayStateShared,
		wake_tx: WakeSender,
		mut stop_rx: watch::Receiver<bool>,
	) {
		tokio::spawn(async move {
			loop {
				tokio::select! {
					cmd = receiver.recv() => match cmd {
						Some(cmd) => {
							Self::resolve_play_command(&player, &state, cmd);
							wake_tx.try_send(()).ok();
						},
						None => break,
					},
					_ = stop_rx.changed() => {
						if *stop_rx.borrow() { break; }
					}
				}
			}
		});
	}

	fn spawn_machine_loop(
		player: PlayerShared,
		state: PlayStateShared,
		mut wake_rx: WakeReceiver,
		mut stop_rx: watch::Receiver<bool>,
	) {
		tokio::spawn(async move {
			loop {
				tokio::select! {
					msg = wake_rx.recv() => {
						if msg.is_none() { break; }
						Self::exec_worker_command(&player, &state).await;
						while wake_rx.try_recv().is_ok() {}
					},
					_ = stop_rx.changed() => {
						if *stop_rx.borrow() { break; }
					}
				}
			}
		});
	}
}

impl Worker {
	fn resolve_play_command(
		player: &PlayerShared,
		state: &PlayStateShared,
		cmd: PlayCommand,
	) {
		match cmd {
			PlayCommand::Play(track, source) => {
				let id = track.id().to_string();

				state.queue.push_back(id, (track, source));
				player.play();
			}
			PlayCommand::Reorder(from, to) => {
				if to < state.queue.len() {
					state.queue.swap_by_index(from, to);
				};
			}
			PlayCommand::Volume(v) => {
				player.set_volume(v.min(1.0).max(0.0));
			}
			PlayCommand::Skip(track) => {
				if !state
					.curr
					.read()
					.as_ref()
					.map(|t| t.id() == track.id())
					.unwrap_or(false)
				{
					state.queue.remove(track.id());
				}
			}

			PlayCommand::Seek(d) => {
				seek_rel(player, d);
			}

			PlayCommand::Shuffle => {
				state.queue.shuffle();
			}
			PlayCommand::SkipAll => {
				state.queue.clear();
			}
			PlayCommand::SkipCurrent => {
				if state.curr.read().is_some() {
					player.clear();
					player.play();

					*state.curr.write() = None;
				} else {
					state.queue.pop_front();
				};
			}
		}
	}
}

impl Worker {
	async fn exec_worker_command(
		player: &PlayerShared,
		state: &PlayStateShared,
	) {
		loop {
			{
				let curr = state.curr.read();
				if curr.is_some() {
					return;
				}
			}

			match state.queue.pop_front() {
				Some((_, (t, source))) => match source.as_ref() {
					PlaySource::StreamFetch(fetcher) => {
						Self::play_from_fetcher(player, state, &t, fetcher)
							.await;
						*state.curr.write() = None;
					}
				},
				None => break,
			}
		}
	}

	async fn play_from_fetcher(
		player: &PlayerShared,
		state: &PlayStateShared,
		track: &TrackShared,
		fetcher: &Fetcher,
	) {
		let stream = match fetcher.fetch_stream(track).await {
			Ok(s) => s,
			Err(_) => return,
		};

		let reader = match super::stream::StreamReader::new(&stream.url).await {
			Ok(r) => r,
			Err(_) => return,
		};

		match Decoder::new(reader) {
			Ok(src) => {
				player.append(src);
				let player = player.clone();
				*state.curr.write() = Some(track.clone());

				tokio::task::spawn_blocking(move || {
					player.sleep_until_end();
				})
				.await
				.ok();
			}
			Err(_) => {}
		}
	}
}

fn seek_rel(player: &PlayerShared, delta: i64) {
	let current = player.get_pos().as_secs() as i64;
	let target = (current + delta).max(0) as u64;
	player.try_seek(Duration::from_secs(target)).ok();
}
