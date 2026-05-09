use std::{
	path::{Path, PathBuf},
	sync::LazyLock,
};

static THIS_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
	dirs::config_dir()
		.unwrap_or_else(|| PathBuf::from("./"))
		.join("nocy")
});

pub fn get_base_path() -> &'static Path {
	&THIS_PATH
}
