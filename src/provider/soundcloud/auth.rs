use std::{
	fs,
	path::PathBuf,
	sync::{Arc, LazyLock},
};

use anyhow::Result;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

static AUTH_PATH: LazyLock<PathBuf> =
	LazyLock::new(|| super::fs::get_cache_path().join("auth"));
static AUTH_FILE_PATH: LazyLock<PathBuf> =
	LazyLock::new(|| AUTH_PATH.join("config.json"));

pub type AuthError<T> = Result<T>;
pub type AuthShared = Arc<AuthorizationData>;
pub type AuthDataSharedRwLock = Arc<RwLock<Option<AuthShared>>>;

#[derive(Serialize, Deserialize)]
pub struct AuthorizationData {
	pub client_id: String,
	pub update_ts: i64,
}

pub struct Authorization {
	inner: AuthDataSharedRwLock,
}

impl Authorization {
	pub fn new() -> Self {
		Self {
			inner: Arc::new(RwLock::new(None)),
		}
	}

	pub fn get_inner(&self) -> &AuthDataSharedRwLock {
		if AUTH_FILE_PATH.exists() {
			if let Ok(text) = fs::read_to_string(AUTH_FILE_PATH.as_path()) {
				if let Ok(data) = serde_json::from_str(&text) {
					self.store(&Arc::new(data)).ok();
				}
			}
		}

		&self.inner
	}
	pub fn store(&self, data: &AuthShared) -> AuthError<()> {
		let s = serde_json::to_string(data.as_ref())?;

		if let Some(pr) = AUTH_FILE_PATH.parent() {
			fs::create_dir_all(&pr).ok();
		}
		fs::write(AUTH_FILE_PATH.as_path(), &s)?;

		*self.inner.write() = Some(data.clone());

		Ok(())
	}
}
