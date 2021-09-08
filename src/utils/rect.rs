extern crate sdl2;

use crate::utils;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
	pub x: i32,
	pub y: i32,
	pub width: u32,
	pub height: u32,
}

impl Rect {

	pub fn new() -> Self {
		Self {
			width: 0, height: 0,
			x: 0, y: 0
		}
	}

	pub fn offset(self, x: i32, y: i32) -> Self {
		Self {
			x, y, width: self.width, height: self.height
		}
	}

	pub fn size(self, width: u32, height: u32) -> Self {
		Self {
			x: self.x,
			y: self.y,
			width, height
		}
	}

	pub fn scalar(self, mul: i32) -> Self {
		Self {
			x: self.x * mul,
			y: self.y * mul,
			width: self.width * mul as u32,
			height: self.width * mul as u32,
		}
	}

	pub fn center(&self) -> utils::Vector2 {
		utils::Vector2::new(
			self.x + (self.width / 2) as i32,
			self.y + (self.height / 2) as i32,
		)
	}
}

impl Into<Option<sdl2::rect::Rect>> for Rect {
	fn into(self) -> Option<sdl2::rect::Rect> {
		Some(sdl2::rect::Rect::new(self.x, self.y, self.width, self.height))
	}
}

#[cfg(test)]
mod test {
use crate::utils::{Rect, Vector2};

	#[test]
	pub fn center() {
		assert_eq!(Vector2::new(1, 1), Rect::new().size(3, 3).center());
		assert_eq!(Vector2::new(2, 2), Rect::new().size(3, 3).offset(1, 1).center());
	}

}