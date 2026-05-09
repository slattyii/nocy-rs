use std::{
	path::{Path, PathBuf},
	sync::LazyLock,
};

static THIS_PATH: LazyLock<PathBuf> =
	LazyLock::new(|| crate::fs::cache::get_cache_path().join("soundcloud"));

pub fn get_cache_path() -> &'static Path {
	&THIS_PATH
}
