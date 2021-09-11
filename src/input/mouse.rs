extern crate sdl2;

use crate::{input::button::{ButtonState}, utils};

const MOUSE_BUTTON_COUNT: usize = 5;

#[derive(PartialEq)]
pub enum MouseButton {
	Left,
	Middle,
	Right,
	X1,
	X2,
}

pub struct Mouse {
	pub x: i32,
	pub y: i32,
	buttons: Vec<ButtonState>,
}

impl Mouse {
	pub fn new() -> Self {
		let mut buttons = Vec::with_capacity(MOUSE_BUTTON_COUNT);
		for _ in 0..MOUSE_BUTTON_COUNT {
			buttons.push(ButtonState::Idle);
		}

		Self {
			x: 0, y: 0,
			buttons
		}
	}

	pub fn get(&self, button: MouseButton) -> ButtonState {
		self.buttons[button as usize]
	}

	pub fn press(&mut self, button: MouseButton) {
		self.buttons[button as usize] = ButtonState::Pressed;
	}

	pub fn release(&mut self, button: MouseButton) {
		self.buttons[button as usize] = ButtonState::Released;
	}

	pub fn step(&mut self) {
		self.buttons = self.buttons.iter_mut().map(|btn| btn.step()).collect();
	}

	pub fn position(&self) -> utils::Vector2 {
		utils::Vector2::new(self.x, self.y)
	}
}

impl MouseButton {
	pub fn from_sdl2(button: sdl2::mouse::MouseButton) -> Option<MouseButton> {
		match button {
			sdl2::mouse::MouseButton::Left =>   Some(MouseButton::Left),
			sdl2::mouse::MouseButton::Right =>  Some(MouseButton::Right),
			sdl2::mouse::MouseButton::Middle => Some(MouseButton::Middle),
			sdl2::mouse::MouseButton::X1 =>     Some(MouseButton::X1),
			sdl2::mouse::MouseButton::X2 =>     Some(MouseButton::X2),
			sdl2::mouse::MouseButton::Unknown => None,
		}
	}
}