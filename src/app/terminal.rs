use std::sync::Arc;
use std::time::Duration;

use crossterm::event::Event;
use crossterm::terminal::{
	EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
	enable_raw_mode,
};
use parking_lot::Mutex;
use ratatui::Terminal;
use ratatui::prelude::CrosstermBackend;

use super::errors::TermError;
use super::types::{Term, TermMuShared};

pub fn term_enter() -> TermError<TermMuShared> {
	enable_raw_mode()?;

	let mut stdout = std::io::stdout();
	crossterm::execute!(stdout, EnterAlternateScreen)?;

	let term = Terminal::new(CrosstermBackend::new(stdout))?;
	Ok(Arc::new(Mutex::new(term)))
}

pub fn term_exit(term: &mut Term) -> TermError<()> {
	disable_raw_mode().ok();
	crossterm::execute!(term.backend_mut(), LeaveAlternateScreen).ok();
	term.show_cursor().ok();

	Ok(())
}

pub fn term_eventread(timeout: u64) -> Option<Event> {
	if crossterm::event::poll(Duration::from_millis(timeout)).ok()? {
		if let Ok(key) = crossterm::event::read() {
			return Some(key);
		}
	}

	None
}
