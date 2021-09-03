extern crate sdl2;

#[derive(PartialEq, Clone, Copy)]
pub enum ButtonState {
	Idle,
	Pressed,
	Held,
	Released,
}

pub struct Button {
	pub state: ButtonState,
	pub key: sdl2::keyboard::Keycode,
}

#[allow(dead_code)]
impl Button {

	pub fn new(key: sdl2::keyboard::Keycode) -> Self {
		Self {
			state: ButtonState::Idle,
			key,
		}
	}

	/// Returns true during the frame the user pressed this button
	pub fn is_pressed(&self) -> bool {
		self.state == ButtonState::Pressed
	}

	/// Returns true while the user presses this button, but not the first frame
	pub fn is_held(&self) -> bool {
		self.state == ButtonState::Held
	}

	/// Returns true during the frame the user released this button
	pub fn is_released(&self) -> bool {
		self.state == ButtonState::Released
	}

	/// Returns true while the user doesn't hold this button, but not for the first frame
	pub fn is_idle(&self) -> bool {
		self.state == ButtonState::Idle
	}

	/// Returns true while the button is down (With first frame)
	pub fn is_down(&self) -> bool {
		self.is_pressed() || self.is_held()
	}

	/// Returns true while the button is up (With first frame)
	pub fn is_up(&self) -> bool {
		self.is_released() || self.is_idle()
	}

	pub fn step(&mut self) {
		if self.state == ButtonState::Pressed {
			self.state = ButtonState::Held;
		}
		if self.state == ButtonState::Released {
			self.state = ButtonState::Idle;	
		}
	}
}