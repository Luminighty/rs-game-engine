extern crate sdl2;


use sdl2::keyboard::Keycode;
use crate::input::button;

#[allow(dead_code)]
pub struct InputSystem {
	pub quit: button::Button,
}



impl InputSystem {
	pub fn init() -> Self {
		Self {
			quit: button::Button::new(Keycode::Escape),
		}
	}
}

impl InputSystem {
	pub fn buttons(&mut self) -> Vec<&mut button::Button> {
		vec![
			&mut self.quit
		]
	}
}


impl InputSystem {
	pub fn update(&mut self) {
		for button in self.buttons() {
			button.step();
		}
	}

	pub fn press_key(&mut self, key: Keycode) {
		for button in self.buttons() {
			if button.key == key {
				button.state = button::ButtonState::Pressed;
			}
		}
	}

	pub fn release_key(&mut self, key: Keycode) {
		for button in self.buttons() {
			if button.key == key {
				button.state = button::ButtonState::Released;
			}
		}
	}
}