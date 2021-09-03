pub struct Application {
	pub is_running: bool,
}

impl Application {
	pub fn new() -> Self {
		Self {
			is_running: true
		}
	}

	pub fn quit(&mut self) {
		self.is_running = false;
	}
}