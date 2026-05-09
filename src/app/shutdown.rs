use super::Nocy;
use super::terminal;

impl Drop for Nocy {
	fn drop(&mut self) {
		terminal::term_exit(&mut self.term.lock()).ok();
	}
}
