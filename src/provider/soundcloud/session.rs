use rand::seq::IndexedRandom;
use reqwest::header::{HeaderMap, HeaderValue};

use super::consts::{ACCEPT_LANGUAGES, USER_AGENTS};
pub fn ss_headers() -> HeaderMap {
	let mut rng = rand::rng();
	let mut map = HeaderMap::new();

	map.insert(
		"User-Agent",
		HeaderValue::from_str(USER_AGENTS.choose(&mut rng).unwrap()).unwrap(),
	);
	map.insert(
		"Accept-Language",
		HeaderValue::from_str(ACCEPT_LANGUAGES.choose(&mut rng).unwrap())
			.unwrap(),
	);
	map.insert(
		"Accept-Encoding",
		HeaderValue::from_static("gzip, deflate, br"),
	);
	map.insert("Connection", HeaderValue::from_static("keep-alive"));

	map
}
