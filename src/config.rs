pub struct Config {
	pub window_title: String,
	pub window_height: u32,
	pub window_width: u32,
}

pub fn default() -> Config {
	Config {
		window_title: String::from("Test game"),
		window_height: 480,
		window_width: 640,
	}
}

