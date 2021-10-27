pub struct Config {
	pub window_title: String,
	pub window_height: u32,
	pub window_width: u32,
	pub icon: Option<String>
}

pub fn default() -> Config {
	Config {
		window_title: String::from("Strategy Card Game"),
		window_width: 640,
		window_height: 480,
		icon: Some(String::from("res/misc/icon.png")),
	}
}

