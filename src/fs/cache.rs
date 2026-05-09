use std::{
	path::{Path, PathBuf},
	sync::LazyLock,
};

static THIS_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
	dirs::cache_dir()
		.map(|f| f.join("nocy"))
		.unwrap_or_else(|| super::base::get_base_path().join("cache"))
});

pub fn get_cache_path() -> &'static Path {
	&THIS_PATH
}
