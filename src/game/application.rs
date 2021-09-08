pub struct Application {
	pub is_running: bool,
	pub frame: usize,
	pub upscale: u32,
}

impl Application {
	pub fn new() -> Self {
		Self {
			is_running: true,
			frame: 0,
			upscale: 2,
		}
	}

	pub fn quit(&mut self) {
		self.is_running = false;
	}
}