use std::{io::Stdout, sync::Arc};

use parking_lot::Mutex;
use ratatui::{Terminal, prelude::CrosstermBackend};

pub type Term = Terminal<CrosstermBackend<Stdout>>;
pub type TermMuShared = Arc<Mutex<Term>>;

