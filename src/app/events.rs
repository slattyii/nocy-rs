use crate::provider::TrackList;

pub enum QuitAgent {
	User(String),
	System(String),
	Error(String),
}

pub enum ResolvedInput {
	Noop,
	Quit(QuitAgent),
}

pub enum UiSearchUpdate {
	SearchResults(TrackList),
}

pub enum UiEvent {
	MainStatus(String),
	Search(UiSearchUpdate),
}
