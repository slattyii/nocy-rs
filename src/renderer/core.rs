use crate::app::stage::Stage;
use crate::version::APP_NAME;
use crate::version::APP_VERSION;

use super::color;
use super::view::RenderView;

use ratatui::style::Stylize;
use ratatui::widgets::Gauge;
use ratatui::widgets::List;
use ratatui::widgets::ListItem;
use ratatui::widgets::ListState;
use ratatui::{
	Frame,
	layout::{Alignment, Constraint, Layout, Rect},
	style::{Modifier, Style},
	text::{Line, Span},
	widgets::{Block, BorderType, Borders, Paragraph},
};
use std::rc::Rc;

pub struct Renderer;

impl Renderer {
	pub fn new() -> Self {
		Self
	}

	pub fn draw(&self, frame: &mut Frame, view: &RenderView) {
		let fr_area = frame.area();
		frame
			.render_widget(Paragraph::new("").bg(color::LAYER_RAISED), fr_area);

		let root = Layout::vertical([
			Constraint::Length(1),
			Constraint::Min(0),
			Constraint::Length(1),
		])
		.split(fr_area);

		self.render_overview(frame, &view, &root);
		self.render_mainview(frame, &view, &root);
	}
}

fn dur_calc(dur: u64) -> (u64, u64, u64) {
	let h = dur / 3600;
	let m = (dur % 3600) / 60;
	let s = dur % 60;

	(h, m, s)
}

fn dur_hmsfmt(dur: u64) -> String {
	let (h, m, s) = dur_calc(dur);
	format!("{h}h {m}m {s}s")
}

fn dur_thmsfmt(dur: u64, long: Option<bool>) -> String {
	let (h, m, s) = dur_calc(dur);

	if h > 0 || long.unwrap_or(false) {
		return format!("{h:0>2}:{m:0>2}:{s:0>2}");
	}

	format!("{m:0>2}:{s:0>2}")
}

fn hint_pill<'a>(key: &'a str, label: &'a str) -> Vec<Span<'a>> {
	vec![
		Span::raw(" "),
		Span::styled(
			key,
			Style::new()
				.fg(color::TEXT_FAINT)
				.bg(color::LAYER_SURFACE)
				.bold(),
		),
		Span::raw(" "),
		Span::styled(label, Style::new().fg(color::TEXT_SECONDARY)),
		Span::raw(" "),
	]
}

impl Renderer {
	fn render_overview(
		&self,
		frame: &mut Frame,
		view: &RenderView,
		root: &Rc<[Rect]>,
	) {
		self.overview_render_topbar(frame, view, root[0]);
		self.overview_render_hintbar(frame, root[2]);
	}

	fn overview_render_topbar(
		&self,
		frame: &mut Frame,
		view: &RenderView,
		area: Rect,
	) {
		let line = Line::from(vec![
			Span::styled(
				APP_NAME,
				Style::default()
					.fg(color::ACCENT_PRIMARY)
					.add_modifier(Modifier::BOLD),
			),
			Span::raw(" "),
			Span::styled(
				APP_VERSION,
				Style::default().fg(color::ACCENT_SECONDARY),
			),
			Span::styled(" | ", Style::default().fg(color::TEXT_FAINT)),
			Span::styled(
				view.app.status,
				Style::default().fg(color::TEXT_SECONDARY),
			),
		]);
		frame.render_widget(Paragraph::new(line), area);
	}

	fn overview_render_hintbar(&self, frame: &mut Frame, area: Rect) {
		let mut spans: Vec<Span> = vec![Span::raw(" ")];

		spans.extend(hint_pill(" Del ", "quit"));
		spans.extend(hint_pill(" / ", "search"));
		spans.extend(hint_pill(" ↑↓ ", "navigate"));
		spans.extend(hint_pill(" Shift ↑↓ ", "reoder"));
		spans.extend(hint_pill(" Ctrl a/z ", "add/remove all"));
		spans.extend(hint_pill(" Ctrl r ", "shuffle board"));
		spans.extend(hint_pill(" Ctrl Alt r ", "shuffle queue"));
		spans.extend(hint_pill(" ~ ", "queue"));

		frame.render_widget(
			Paragraph::new(Line::from(spans)).alignment(Alignment::Center),
			area,
		);
	}
}

impl Renderer {
	fn render_mainview(
		&self,
		frame: &mut Frame,
		view: &RenderView,
		root: &Rc<[Rect]>,
	) {
		let body =
			Layout::horizontal([Constraint::Min(0), Constraint::Length(48)])
				.split(root[1]);

		let left =
			Layout::vertical([Constraint::Length(3), Constraint::Min(0)])
				.split(body[0]);

		self.mainview_render_searchbar(frame, view, left[0]);
		self.mainview_render_board(frame, view, left[1]);
		self.mainview_render_sidebar(frame, view, body[1]);
	}

	fn mainview_render_searchbar(
		&self,
		frame: &mut Frame,
		view: &RenderView,
		area: Rect,
	) {
		let active = matches!(view.app.stage, Stage::Search);
		let fg_col = if active {
			color::ACCENT_PRIMARY
		} else {
			color::ACCENT_SECONDARY
		};
		let border_col = if active {
			color::BORDER_ACTIVE
		} else {
			color::BORDER
		};
		let text = if active {
			view.search.query
		} else {
			view.play
				.current
				.as_ref()
				.map(|t| t.title())
				.unwrap_or("Nice Choice!")
		};

		let inner = Line::from(vec![
			Span::styled(
				" ",
				Style::default().fg(fg_col).add_modifier(Modifier::BOLD),
			),
			Span::styled(
				text,
				Style::default().fg(if active {
					color::TEXT_PRIMARY
				} else {
					color::TEXT_FAINT
				}),
			),
			if active {
				Span::styled(
					"▎",
					Style::default()
						.fg(fg_col)
						.add_modifier(Modifier::SLOW_BLINK),
				)
			} else {
				Span::styled(" ", Style::default())
			},
		]);

		let block = Block::default()
			.borders(Borders::ALL)
			.border_type(BorderType::Rounded)
			.border_style(Style::default().fg(border_col))
			.title(Line::from(vec![
				Span::styled("", Style::default().fg(fg_col)),
				Span::styled(
					view.app.title,
					Style::default()
						.fg(color::TEXT_SECONDARY)
						.add_modifier(Modifier::BOLD),
				),
				Span::raw(" "),
			]))
			.title_alignment(Alignment::Left);

		frame.render_widget(Paragraph::new(inner).block(block), area);
	}

	fn mainview_render_board(
		&self,
		frame: &mut Frame,
		view: &RenderView,
		area: Rect,
	) {
		let active = matches!(view.app.stage, Stage::Play);
		let border_col = if active {
			color::BORDER_ACTIVE
		} else {
			color::BORDER
		};

		let block = Block::default()
			.borders(Borders::ALL)
			.border_type(BorderType::Rounded)
			.border_style(Style::default().fg(border_col))
			.title(Span::styled(
				" Board ",
				Style::default().fg(color::TEXT_FAINT),
			));

		let arr = &view.board.items;
		if arr.is_empty() {
			self.mainview_render_board_empty(frame, block, view, area);
			return;
		}

		let height = area.height as usize;
		let start = view.board.selected.saturating_sub(height / 2);
		let end = (start + height).min(arr.len());

		let mut state = ListState::default();
		state.select(Some(view.board.selected - start));

		let items = arr[start..end]
			.iter()
			.enumerate()
			.map(|(i, t)| {
				let i = start + i;
				let is_queued = view.board.queue.contains_key(t.id());
				let is_playing = view
					.play
					.current
					.as_ref()
					.map(|ct| ct.id() == t.id())
					.unwrap_or(false);

				let (pfx, sty, fg) = match (is_playing, is_queued) {
					(true, _) => (
						"▶ ",
						Style::default().fg(color::ACCENT_PRIMARY),
						color::ACCENT_PRIMARY,
					),
					(_, true) => (
						"# ",
						Style::default().fg(color::ACCENT_SECONDARY),
						color::ACCENT_SECONDARY,
					),
					_ => (
						"   ",
						Style::default().fg(color::TEXT_FAINT),
						color::TEXT_SECONDARY,
					),
				};

				ListItem::new(Line::from(vec![
					Span::styled(pfx, sty),
					Span::styled(
						format!("{:2}. ", i + 1),
						Style::default().fg(color::TEXT_FAINT),
					),
					Span::raw(" "),
					Span::styled(t.title(), Style::default().fg(fg)),
					Span::raw(" "),
					Span::styled(
						t.provider().name(),
						Style::default().fg(color::TEXT_FAINT),
					),
					Span::raw(" "),
					Span::styled(
						dur_thmsfmt(t.duration().as_secs(), None),
						Style::default().fg(color::TEXT_FAINT),
					),
				]))
			})
			.collect::<Vec<ListItem>>();

		frame.render_stateful_widget(
			List::new(items).block(block).highlight_style(
				Style::default()
					.bg(color::LAYER_SELECT)
					.fg(color::TEXT_PRIMARY),
			),
			area,
			&mut state,
		);
	}

	fn mainview_render_board_empty(
		&self,
		frame: &mut Frame,
		block: Block,
		_view: &RenderView,
		area: Rect,
	) {
		const ART: &[(&str, bool)] = &[
			("      /\\       ", false),
			("     /  \\      ", false),
			("    / __ \\     ", false),
			("   /______\\    ", false),
			("   (˶ᵔ ᵕ ᵔ˶)   ", true),
			("    ╰─────╯    ", false),
			("    ╭──┬──╮    ", false),
			("    │  ♫  │    ", true),
			("    ╰──┬──╯    ", false),
			("      / \\      ", false),
			("     /   \\     ", false),
		];

		let inner_h = area.height.saturating_sub(2);
		let art_h = ART.len() as u16 + 2;
		let pad = (inner_h.saturating_sub(art_h)) / 2;
		let mut lines: Vec<Line> = (0..pad).map(|_| Line::from("")).collect();

		for (row, is_face) in ART {
			let fg = if *is_face {
				color::TEXT_SECONDARY
			} else {
				color::TEXT_FAINT
			};
			lines.push(Line::from(Span::styled(*row, Style::default().fg(fg))));
		}

		lines.push(Line::from(""));
		lines.push(Line::from(Span::styled(
			"♪  nay nghe gì đó",
			Style::default()
				.fg(color::TEXT_FAINT)
				.add_modifier(Modifier::ITALIC),
		)));

		frame.render_widget(
			Paragraph::new(lines)
				.alignment(Alignment::Center)
				.block(block),
			area,
		);
	}

	fn mainview_render_sidebar(
		&self,
		frame: &mut Frame,
		view: &RenderView,
		area: Rect,
	) {
		let lay =
			Layout::vertical([Constraint::Length(10), Constraint::Min(0)])
				.split(area);

		self.mainview_render_sidebar_playing(frame, view, lay[0]);
		self.mainview_render_sidebar_queue(frame, view, lay[1]);
	}

	fn mainview_render_sidebar_playing(
		&self,
		frame: &mut Frame,
		view: &RenderView,
		area: Rect,
	) {
		let title = if view.play.current.is_none() {
			" Nothing "
		} else {
			if view.play.paused {
				" Paused "
			} else {
				" Playing "
			}
		};

		let block = Block::new()
			.borders(Borders::ALL)
			.border_type(BorderType::Rounded)
			.border_style(color::BORDER)
			.title(Span::styled(
				title,
				Style::default().fg(color::TEXT_SECONDARY),
			));

		let Some(np) = &view.play.current else {
			frame.render_widget(
				Paragraph::new(Line::styled(
					"  ♪  nothing playing",
					Style::default().fg(color::TEXT_FAINT),
				))
				.block(block),
				area,
			);
			return;
		};

		let inner = Layout::vertical([
			Constraint::Min(0),
			Constraint::Length(2),
			Constraint::Length(1),
			Constraint::Length(2), // gap
			Constraint::Length(1),
			Constraint::Length(1),
			Constraint::Min(0),
		])
		.margin(1)
		.split(area);

		let is_paused = view.play.paused;

		let accent = if !is_paused {
			color::ACCENT_PRIMARY
		} else {
			color::ACCENT_SECONDARY
		};

		frame.render_widget(block, area);
		frame.render_widget(
			Paragraph::new(Line::from(Span::styled(
				np.title(),
				Style::default().fg(accent).add_modifier(Modifier::BOLD),
			)))
			.alignment(Alignment::Center),
			inner[1],
		);
		frame.render_widget(
			Paragraph::new(Line::from(Span::styled(
				np.artist(),
				Style::default().fg(color::TEXT_FAINT),
			)))
			.alignment(Alignment::Center),
			inner[2],
		);

		let gauge_area = Layout::horizontal([
			Constraint::Length(2),
			Constraint::Min(0),
			Constraint::Length(2),
		])
		.split(inner[4]);

		let curr_secs = view.play.pos.as_secs();
		let np_secs = np.duration().as_secs();
		let pct = (curr_secs as f32 / np_secs as f32 * 100.0) as u16;

		frame.render_widget(
			Gauge::default()
				.gauge_style(Style::default().fg(accent).bg(color::TEXT_FAINT))
				.percent(pct.min(100).max(0))
				.label(""),
			gauge_area[1],
		);

		let lay = Layout::horizontal([
			Constraint::Min(0),
			Constraint::Length(8),
			Constraint::Length(3),
			Constraint::Length(8),
			Constraint::Length(1),
			Constraint::Length(8),
			Constraint::Length(3),
			Constraint::Length(8),
			Constraint::Min(0),
		])
		.split(inner[5]);

		let fg_col = color::TEXT_FAINT;
		let ac_col = color::TEXT_SECONDARY;
		let curr_dur = dur_thmsfmt(np_secs, None);

		frame.render_widget(
			Paragraph::new(Line::styled(
				format!("vol {}%", (view.play.vol * 100.0) as usize),
				Style::default().fg(fg_col),
			)),
			lay[1],
		);
		frame.render_widget(
			Paragraph::new(Line::styled(
				dur_thmsfmt(curr_secs, Some(curr_dur.len() == 8)),
				Style::default().fg(ac_col),
			)),
			lay[3],
		);
		frame.render_widget(
			Paragraph::new(Line::styled(
				format!("{}", curr_dur),
				Style::default().fg(ac_col),
			)),
			lay[5],
		);
		frame.render_widget(
			Paragraph::new(Line::styled(APP_NAME, Style::default().fg(fg_col))),
			lay[7],
		);
	}

	fn mainview_render_sidebar_queue(
		&self,
		frame: &mut Frame,
		view: &RenderView,
		area: Rect,
	) {
		let pending = view.board.queue;
		let block = Block::new()
			.borders(Borders::ALL)
			.border_type(BorderType::Rounded)
			.border_style(color::BORDER)
			.title(Span::styled(
				format!(
					" Queue | {} ",
					dur_hmsfmt(
						pending
							.iter()
							.map(|t| t.1.0.duration().as_secs())
							.sum()
					)
				),
				Style::default().fg(color::TEXT_SECONDARY),
			));

		if pending.is_empty() {
			frame.render_widget(
				Paragraph::new(Line::styled(
					"  ♪  nothing in queue",
					Style::default().fg(color::TEXT_FAINT),
				))
				.block(block),
				area,
			);
			return;
		};

		let take_n = area.height.saturating_sub(4) as usize;
		let mut items = pending
			.iter()
			.take(take_n)
			.enumerate()
			.map(|(i, pd)| {
				let track = &pd.1.0;
				let is_next = i == 0;

				let style = Style::default().fg(if is_next {
					color::ACCENT_PRIMARY
				} else {
					color::TEXT_FAINT
				});

				let prefix = if is_next { "▷ " } else { "  " };
				let mut chars = track.title().chars();
				let head: String = chars
					.by_ref()
					.take(area.width.saturating_sub(2) as usize)
					.collect();
				let title = if chars.next().is_some() {
					format!("{}…", head)
				} else {
					head
				};

				ListItem::new(Line::from(vec![
					Span::styled(prefix, style),
					Span::styled(title, style),
				]))
			})
			.collect::<Vec<ListItem>>();

		let len = pending.len();
		if len > take_n {
			let style = Style::default().fg(color::TEXT_SECONDARY);
			items.push(ListItem::new(Line::from(vec![
				Span::styled("  ", style),
				Span::styled(format!(" - {}+", len - take_n), style),
			])));
		}

		frame.render_widget(List::new(items).block(block), area);
	}
}
