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

	pub fn contains(&self, vector: &utils::Vector2) -> bool {
		self.x <= vector.x &&
		self.x + self.width as i32 >= vector.x &&
		self.y <= vector.y &&
		self.y + self.height as i32 >= vector.y
	}

	pub fn expand(&self, unit: i32) -> Self {
		Self {
			x: self.x - unit,
			y: self.y - unit,
			width:  (self.width as i32 + unit * 2) as u32,
			height: (self.height as i32 + unit * 2) as u32,
		}
	}

	pub fn shrink(&self, unit: i32) -> Self {
		self.expand(-unit)
	}
}

impl Into<Option<sdl2::rect::Rect>> for Rect {
	fn into(self) -> Option<sdl2::rect::Rect> {
		Some(sdl2::rect::Rect::new(self.x, self.y, self.width, self.height))
	}
}

impl Into<sdl2::rect::Rect> for Rect {
	fn into(self) -> sdl2::rect::Rect {
		sdl2::rect::Rect::new(self.x, self.y, self.width, self.height)
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

	#[test]
	pub fn contains() {
		assert!(Rect::new().size(2, 2).contains(&Vector2::new(1, 1)));
		assert!(Rect::new().size(2, 2).contains(&Vector2::new(0, 0)));
		assert!(Rect::new().size(2, 2).contains(&Vector2::new(2, 0)));
		assert!(Rect::new().size(2, 2).contains(&Vector2::new(0, 2)));
		assert!(Rect::new().size(2, 2).contains(&Vector2::new(2, 2)));

		assert!(!Rect::new().size(2, 2).contains(&Vector2::new(-1, 0)));
		assert!(!Rect::new().size(2, 2).contains(&Vector2::new(0, -1)));
		assert!(!Rect::new().size(2, 2).contains(&Vector2::new(-1, -1)));
		assert!(!Rect::new().size(2, 2).contains(&Vector2::new(3, 0)));
		assert!(!Rect::new().size(2, 2).contains(&Vector2::new(0, 3)));
		assert!(!Rect::new().size(2, 2).contains(&Vector2::new(3, 3)));
		
		assert!(Rect::new().offset(1, 1).size(2, 2).contains(&Vector2::new(2, 2)));
		assert!(Rect::new().offset(1, 1).size(2, 2).contains(&Vector2::new(1, 1)));
		assert!(Rect::new().offset(1, 1).size(2, 2).contains(&Vector2::new(3, 1)));
		assert!(Rect::new().offset(1, 1).size(2, 2).contains(&Vector2::new(1, 3)));
		assert!(Rect::new().offset(1, 1).size(2, 2).contains(&Vector2::new(3, 3)));

		assert!(!Rect::new().offset(1, 1).size(2, 2).contains(&Vector2::new(1, 0)));
		assert!(!Rect::new().offset(1, 1).size(2, 2).contains(&Vector2::new(0, 1)));
		assert!(!Rect::new().offset(1, 1).size(2, 2).contains(&Vector2::new(0, 0)));
		assert!(!Rect::new().offset(1, 1).size(2, 2).contains(&Vector2::new(4, 0)));
		assert!(!Rect::new().offset(1, 1).size(2, 2).contains(&Vector2::new(0, 4)));
		assert!(!Rect::new().offset(1, 1).size(2, 2).contains(&Vector2::new(4, 4)));
	}

}