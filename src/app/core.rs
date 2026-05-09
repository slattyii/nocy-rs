use std::process;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, OnceLock, mpsc};
use std::time::{Duration, Instant};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use rand::seq::SliceRandom;

use crate::player::{Fetcher, PlaySource, Playerith};
use crate::provider::{
	Provider, ProviderType, SearchOptions, SoundCloudProvider, TrackList,
	TrackShared,
};
use crate::renderer::{
	AppView, BoardView, PlayView, RenderView, Renderer, SearchView,
};

use super::errors::AppError;
use super::events::{QuitAgent, ResolvedInput, UiEvent, UiSearchUpdate};
use super::stage::Stage;
use super::terminal;
use super::types::TermMuShared;

struct Search {
	query: String,
}

struct Board {
	store: Option<(usize, TrackList)>,
	items: TrackList,
	selected: usize,
	show_queue: bool,
}

struct Providers {
	selected: ProviderType,
	soundcloud: OnceLock<Arc<SoundCloudProvider>>,
}

pub struct Nocy {
	// Nocy Music Player
	pub(super) term: TermMuShared,

	stage: Stage,
	status: String,
	title: String,

	search: Search,
	board: Board,
	providers: Providers,

	uievent_rx: Receiver<UiEvent>,
	uievent_tx: Sender<UiEvent>,

	playerith: Arc<Playerith>,
	renderer: Renderer,
}

impl Nocy {
	pub fn init() -> Self {
		let term = match terminal::term_enter() {
			Ok(t) => t,
			Err(e) => {
				eprintln!("cannot get any output device: {}", e.to_string());
				process::exit(1);
			}
		};
		let playerith = match Playerith::init() {
			Ok(p) => Arc::new(p),
			Err(e) => {
				eprintln!("cannot get any output device: {}", e.to_string());
				process::exit(1);
			}
		};
		let renderer = Renderer::new();
		let (tx, rx) = mpsc::channel();

		Self {
			term,

			stage: Stage::Search,
			status: " 404: Silence Not Found ".into(),
			title: " Nocy Nocy ".into(),

			search: Search {
				query: String::new(),
			},
			board: Board {
				store: None,
				items: Vec::default(),
				selected: 0,
				show_queue: false,
			},
			providers: Providers {
				selected: ProviderType::SoundCloud,
				soundcloud: OnceLock::new(),
			},

			uievent_tx: tx,
			uievent_rx: rx,

			playerith,
			renderer,
		}
	}

	pub async fn run(mut self) -> AppError<()> {
		self.exec_event_loop().await;

		Ok(())
	}
}

// Event Loop
impl Nocy {
	async fn exec_event_loop(&mut self) {
		const POLL_TIMEOUT_DEFAULT: u64 = 1000 / 60;

		let idle_timeout = Duration::from_secs(3);
		let mut idle_inst = Instant::now();
		let mut is_idle = false;

		let mut poll_timeout = POLL_TIMEOUT_DEFAULT;

		loop {
			self.exec_drain_events();
			self.exec_data_update();

			{
				let view = RenderView {
					app: AppView {
						stage: &self.stage,
						title: &self.title,
						status: &self.status,
					},
					search: SearchView {
						query: &self.search.query,
					},
					board: BoardView {
						queue: &self.playerith.queue_snapshot(),
						items: &self.board.items,
						selected: self.board.selected,
					},
					play: PlayView {
						current: &self.playerith.current(),
						pos: self.playerith.current_pos(),
						paused: self.playerith.current_paused(),
						vol: self.playerith.current_vol(),
					},
				};
				self.term.lock().draw(|f| self.renderer.draw(f, &view)).ok();
			}

			if let Some(e) = terminal::term_eventread(poll_timeout) {
				if is_idle {
					idle_inst = Instant::now();
					poll_timeout = POLL_TIMEOUT_DEFAULT;
					is_idle = false;
				}

				let ret = match e {
					Event::Key(key) => self.resolve_input_event(key).await,
					_ => ResolvedInput::Noop,
				};

				match ret {
					ResolvedInput::Quit(_) => break,
					_ => {}
				}
			}

			if !is_idle && idle_inst.elapsed() >= idle_timeout {
				is_idle = true;
				poll_timeout = 950;
			}
		}
	}
}

// Event
impl Nocy {
	fn exec_drain_events(&mut self) {
		while let Ok(e) = self.uievent_rx.try_recv() {
			match e {
				UiEvent::MainStatus(s) => {
					self.status = s;
				}
				UiEvent::Search(UiSearchUpdate::SearchResults(v)) => {
					self.stage = Stage::Play;
					self.status = format!("Found {} Tracks", v.len());
					self.board.items = v;
				}
			}
		}
	}

	fn exec_data_update(&mut self) {
		if self.board.show_queue {
			let snapshot = self.playerith.queue_snapshot();
			if snapshot.len() == 0 {
				self.handle_show_queue(false);
			} else {
				if snapshot.len() != self.board.items.len() {
					self.board.items.retain(|e| snapshot.contains_key(e.id()));
				}
			}
		}
	}
}

// Input
impl Nocy {
	async fn resolve_input_event(&mut self, key: KeyEvent) -> ResolvedInput {
		match key.code {
			KeyCode::Delete => {
				return ResolvedInput::Quit(QuitAgent::User(
					"user quit".into(),
				));
			}

			KeyCode::Esc => return self.handle_back(),

			KeyCode::Enter if matches!(self.stage, Stage::Search) => {
				self.handle_search();
			}
			KeyCode::Enter if matches!(self.stage, Stage::Play) => {
				self.handle_select();
			}

			KeyCode::Char('/') => {
				self.handle_toggle_search();
			}
			KeyCode::Char('a') | KeyCode::Char('z')
				if key.modifiers.contains(KeyModifiers::CONTROL) =>
			{
				if matches!(key.code, KeyCode::Char('a')) {
					self.handle_add_all();
				} else {
					self.handle_remove_all();
				}
			}
			KeyCode::Char('r')
				if key.modifiers.contains(KeyModifiers::CONTROL) =>
			{
				self.handle_shuffle(key.modifiers.contains(KeyModifiers::ALT));
			}
			KeyCode::Char('~') if matches!(self.stage, Stage::Play) => {
				self.handle_show_queue(!self.board.show_queue);
			}
			KeyCode::Char('-') | KeyCode::Char('=') => {
				self.handle_vol(matches!(key.code, KeyCode::Char('=')))
			}
			KeyCode::Char(' ') if matches!(self.stage, Stage::Play) => {
				self.handle_pause();
			}

			KeyCode::Char(c) => self.handle_input_char(Some(c)),
			KeyCode::Backspace => self.handle_input_char(None),

			KeyCode::Up | KeyCode::Down
				if matches!(self.stage, Stage::Play) =>
			{
				let is_up = matches!(key.code, KeyCode::Up);

				if key.modifiers.contains(KeyModifiers::SHIFT) {
					self.handle_reorder(is_up);
				} else {
					self.handle_navigate(is_up);
				}
			}
			KeyCode::Left | KeyCode::Right => {
				self.handle_seek(matches!(key.code, KeyCode::Left));
			}

			_ => {}
		}

		ResolvedInput::Noop
	}
}

// Provider
impl Nocy {
	fn get_provider(&self, provider_type: ProviderType) -> Arc<dyn Provider> {
		match provider_type {
			ProviderType::SoundCloud => self
				.providers
				.soundcloud
				.get_or_init(|| Arc::new(SoundCloudProvider::new()))
				.clone(),
		}
	}

	fn submit_with_fetcher(&self, track: &TrackShared) {
		let provider = self.get_provider(self.providers.selected);

		let fetcher = {
			Fetcher::new(move |track| {
				let provider = provider.clone();

				{
					let track = track.clone();
					async move {
						match track.stream() {
							Some(s) => Ok(s),
							None => provider.stream(&track).await,
						}
					}
				}
			})
		};

		self.playerith
			.submit(&track, Arc::new(PlaySource::StreamFetch(fetcher)));
	}
}

// Handle
impl Nocy {
	fn handle_show_queue(&mut self, enable: bool) {
		if enable {
			if self.board.show_queue {
				return;
			}

			let snapshot = self.playerith.queue_snapshot();
			if snapshot.is_empty() {
				return;
			}

			let selected = self.board.selected;
			let new_items = snapshot
				.iter()
				.map(|p| p.1.0.clone())
				.collect::<TrackList>();

			let store_selected = std::mem::replace(
				&mut self.board.selected,
				selected.min(new_items.len() - 1),
			);
			let store_items =
				std::mem::replace(&mut self.board.items, new_items);

			self.board.show_queue = true;
			self.board.store = Some((store_selected, store_items));
		} else {
			if !self.board.show_queue {
				return;
			}

			self.board.show_queue = false;
			if let Some((selected, items)) = self.board.store.take() {
				self.board.selected = selected;
				self.board.items = items;
			}
		}
	}

	fn handle_back(&mut self) -> ResolvedInput {
		if matches!(self.stage, Stage::Wait) {
			return ResolvedInput::Noop;
		}

		self.handle_show_queue(false);

		if matches!(self.stage, Stage::Play) {
			if self.playerith.current().is_some()
				|| self.playerith.queue_snapshot().len() > 0
			{
				self.playerith.skip_current();
			} else {
				self.stage = Stage::Search;
				self.board.items.clear();
			};
		} else {
			if !self.search.query.is_empty() {
				self.search.query.clear();
			} else {
				return ResolvedInput::Quit(QuitAgent::User(
					"user back pressed".into(),
				));
			};
		};

		ResolvedInput::Noop
	}

	fn handle_pause(&self) {
		if self.playerith.current_paused() {
			self.playerith.current_resume();
		} else {
			self.playerith.current_pause();
		}
	}

	fn handle_vol(&self, up: bool) {
		let vol = self.playerith.current_vol();
		self.playerith.set_vol(
			(if up { vol + 0.05 } else { vol - 0.05 }).min(1.0).max(0.0),
		);
	}

	fn handle_toggle_search(&mut self) {
		if matches!(self.stage, Stage::Wait) {
			return;
		}
		if matches!(self.stage, Stage::Play) {
			self.stage = Stage::Search;
		} else if matches!(self.stage, Stage::Search)
			&& !self.board.items.is_empty()
		{
			self.stage = Stage::Play;
		}
	}

	fn handle_search(&mut self) {
		self.handle_show_queue(false);

		let query = std::mem::take(&mut self.search.query);
		let provider = self.get_provider(self.providers.selected);
		let uievent_tx = self.uievent_tx.clone();

		self.stage = Stage::Wait;

		tokio::spawn(async move {
			let _ = match provider
				.search(
					&query,
					Some(&SearchOptions {
						limit: 50,
					}),
				)
				.await
			{
				Ok(v) => uievent_tx
					.send(UiEvent::Search(UiSearchUpdate::SearchResults(v))),
				Err(e) => uievent_tx.send(UiEvent::MainStatus(e.to_string())),
			};
		});
	}

	fn handle_shuffle(&mut self, alt: bool) {
		if !alt {
			let mut rng = rand::rng();
			self.board.items.shuffle(&mut rng);
		} else {
			self.playerith.shuffle();
		}
	}

	fn handle_add_all(&self) {
		let snapshot = self.playerith.queue_snapshot();
		for t in self.board.items.iter().filter(|t| {
			!snapshot.contains_key(t.id())
				&& !self
					.playerith
					.current()
					.map(|c| c.id() == t.id())
					.unwrap_or(false)
		}) {
			self.submit_with_fetcher(&t);
		}
	}

	fn handle_remove_all(&self) {
		self.playerith.skip_all();
	}

	fn handle_select(&mut self) {
		let selected_track = &self.board.items[self.board.selected];
		if self
			.playerith
			.queue_snapshot()
			.contains_key(selected_track.id())
		{
			self.playerith.skip(&selected_track);
			return;
		}

		self.submit_with_fetcher(&selected_track);
	}

	fn handle_input_char(&mut self, ch: Option<char>) {
		if let Some(c) = ch {
			self.search.query.push(c);
		} else {
			self.search.query.pop();
		}
	}

	fn handle_seek(&self, left: bool) {
		if left {
			self.playerith.seek(-5);
		} else {
			self.playerith.seek(5);
		}
	}

	fn handle_reorder(&mut self, up: bool) {
		let selected = self.board.selected;
		let to = if up {
			selected
				.checked_sub(1)
				.unwrap_or(self.board.items.len() - 1)
		} else {
			(selected + 1) % self.board.items.len()
		};

		self.board.items.swap(selected, to);
		if self.board.show_queue {
			self.playerith.reorder(selected, to);
		}

		self.board.selected = to;
	}

	fn handle_navigate(&mut self, up: bool) {
		self.board.selected = if up {
			self.board
				.selected
				.checked_sub(1)
				.unwrap_or(self.board.items.len() - 1)
		} else {
			(self.board.selected + 1) % self.board.items.len()
		};
	}
}
