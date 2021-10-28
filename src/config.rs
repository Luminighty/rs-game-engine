pub struct Config {
	pub window_title: String,
	pub window_size: (u32, u32),
	pub window_min_size: (u32, u32),
	pub icon: Option<String>
}

pub fn default() -> Config {
	Config {
		window_title: String::from("Strategy Card Game"),
		window_size: (640, 480),
		window_min_size: (20 * 16, 15 * 16),
		icon: Some(String::from("res/misc/icon.png")),
	}
}

