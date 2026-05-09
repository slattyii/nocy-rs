use crate::app::stage::Stage;

pub struct AppView<'a> {
	pub stage: &'a Stage,
	pub title: &'a str,
	pub status: &'a str,
}
