extern crate sdl2;

use crate::utils::{self, Vector2};

#[derive(Clone, Copy, PartialEq)]
pub enum Sprite {
	Player

}

pub struct Sprites {

}

impl Sprites {
	pub fn init() -> Self {
		Self {
		}
	}

	pub fn fields(&mut self) -> Vec<&mut SpriteData> {
		vec![
		]
	}
}

pub struct SpriteData {
	pub path: String,
	pub rect: utils::Rect,
	pub padding: Option<utils::Vector2>,
}

impl SpriteData {
	pub fn sprite(path: &str, rect: utils::Rect) -> Self {
		Self {
			path: String::from(path),
			padding: None,
			rect,
		}
	}

	pub fn sheet(path: &str, rect: utils::Rect, padding: Vector2) -> Self {
		Self {
			path: String::from(path),
			rect, padding: Some(padding)
		}
	}

	pub fn get_rect(&self, x: i32, y: i32) -> utils::Rect {
		if let Some(padding) = self.padding {
			self.rect.clone()
			.offset(self.rect.x + (padding.x + self.rect.width as i32) * x, 
					self.rect.y + (padding.y + self.rect.height as i32) * y)
		} else {
			self.rect.clone()
		}
	}
}



#[cfg(test)]
mod test {
	use crate::render::SpriteData;
	use crate::utils;

	#[test]
	pub fn get_rect() {
		let rect = utils::Rect::new().size(10, 10).offset(1, 1);
		let sprite = SpriteData::sprite("", rect.clone());
		let sprite_sheet = SpriteData::sheet("", rect.clone(), utils::Vector2::new(5, 5));
		assert_eq!(rect, sprite.get_rect(2, 2));
		assert_eq!(rect, sprite_sheet.get_rect(0, 0));
		assert_eq!(utils::Rect::new().size(10, 10).offset(31, 1), sprite_sheet.get_rect(2, 0));
		assert_eq!(utils::Rect::new().size(10, 10).offset(1, 31), sprite_sheet.get_rect(0, 2));
	}

}