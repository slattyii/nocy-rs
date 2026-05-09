use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version {
	pub major: u8,
	pub minor: u8,
	pub patch: u8,
	pub name: &'static str,
}

impl Version {
	pub const fn new(
		major: u8,
		minor: u8,
		patch: u8,
		name: &'static str,
	) -> Self {
		Self {
			major,
			minor,
			patch,
			name,
		}
	}
}

impl fmt::Display for Version {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"v{}.{}.{} {}",
			self.major, self.minor, self.patch, self.name
		)
	}
}

pub const APP_NAME: &'static str = "nocy";
pub const APP_VERSION_IT: Version = Version::new(0, 0, 1, "internal");
pub const APP_VERSION: &'static str = const_format::formatcp!(
	"v{}.{}.{} {}",
	APP_VERSION_IT.major,
	APP_VERSION_IT.minor,
	APP_VERSION_IT.patch,
	APP_VERSION_IT.name
);
