extern crate sdl2;

use crate::utils::{self, Rect, Vector2};

#[derive(Clone, Copy, PartialEq)]
pub enum Sprite {
	Player,
	Undead,
	Ground,
	Wall,
	Interact,
}

pub struct Sprites {
	player: SpriteData,
	undead: SpriteData,
	ground: SpriteData,
	wall: SpriteData,
	interact: SpriteData,
}

impl Sprites {
	pub fn init() -> Self {
		let actor_rect = Rect::new().size(16, 16);
		Self {
			player:   SpriteData::sheet("res/actor/player.png", actor_rect),
			undead:   SpriteData::sheet("res/actor/undead.png", actor_rect),
			ground:   SpriteData::sheet("res/map/ground.png",   actor_rect),
			wall:     SpriteData::sheet("res/map/wall.png",     actor_rect),
			interact: SpriteData::sheet("res/ui/interact.png",  actor_rect),
		}
	}

	pub fn fields_mut(&mut self) -> Vec<&mut SpriteData> {
		vec![
			&mut self.player, 
			&mut self.undead,
			&mut self.ground, 
			&mut self.wall,
			&mut self.interact,
		]
	}

	pub fn fields(&self) -> Vec<&SpriteData> {
		vec![
			&self.player, 
			&self.undead,
			&self.ground, 
			&self.wall,
			&self.interact,
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

	pub fn sheet(path: &str, rect: utils::Rect) -> Self {
		Self {
			path: String::from(path),
			rect, 
			padding: Some(Vector2::zero())
		}
	}

	pub fn padding(mut self, padding: Vector2) -> Self {
		self.padding = Some(padding);
		self
	}

	pub fn get_rect(&self, x: u8, y: u8) -> utils::Rect {
		if let Some(padding) = self.padding {
			self.rect.clone()
			.offset(self.rect.x + (padding.x + self.rect.width as i32) * x as i32, 
					self.rect.y + (padding.y + self.rect.height as i32) * y as i32)
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
		let sprite_sheet = SpriteData::sheet("", rect).padding(utils::Vector2::new(5, 5));
		assert_eq!(rect, sprite.get_rect(2, 2));
		assert_eq!(rect, sprite_sheet.get_rect(0, 0));
		assert_eq!(utils::Rect::new().size(10, 10).offset(31, 1), sprite_sheet.get_rect(2, 0));
		assert_eq!(utils::Rect::new().size(10, 10).offset(1, 31), sprite_sheet.get_rect(0, 2));
	}

}