use super::app::AppView;
use super::board::BoardView;
use super::play::PlayView;
use super::search::SearchView;

pub struct RenderView<'a> {
	pub app: AppView<'a>,
	pub search: SearchView<'a>,
	pub board: BoardView<'a>,
	pub play: PlayView<'a>,
}
