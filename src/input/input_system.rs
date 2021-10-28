extern crate sdl2;


use std::slice::Iter;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::input::button;
use crate::input::mouse;
use crate::utils::EventSystem;


#[allow(dead_code)]
pub struct InputSystem {
	_events: EventSystem<Event>,
	pub quit: button::Button,
	pub mouse: mouse::Mouse,
}



impl InputSystem {
	pub fn init() -> Self {
		Self {
			_events: EventSystem::new(),
			quit: button::Button::new(Keycode::Escape),
			mouse:  mouse::Mouse::new(),
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
		self.mouse.step();
		self._events.clear();
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

	pub fn events(&self) -> Iter<Event> {
		self._events.iter()
	}

	pub fn push_event(&mut self, event: Event) {
		self._events.push_event(event)
	}
}