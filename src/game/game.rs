

use crate::{game::map::Map, utils::Vector2};

use super::{actor::{Enemy, Player}, map::Tile};

pub struct Game {
	pub map: Map,
	pub player: Player,
	pub enemies: Vec<Enemy>,

	pub animation_step: usize,

}


impl Game {
	pub fn occupied_tiles(&self) -> Vec<bool> {
		let size = self.map.get_size();
		let mut tiles = vec![false; (size.x * size.y) as usize];

		for (pos, _) in self.map.walls.iter() {
			tiles[(pos.x + (pos.y * size.x)) as usize] = true;
		}

		for i in 0..self.map.tiles.len() {
			if self.map.tiles[i] == Tile::Void {
				tiles[i] = true;
			}
		}

		tiles
	}
}

impl Game {
	pub fn new() -> Self {
		Self {
			animation_step: 30,
			map: Map::example_map(),
			player: Player::new(Vector2::new(5, 6)),
			enemies: vec![
				// Enemy::bat(Vector2::new(3, 4)),
				// Enemy::bat(Vector2::new(5, 4)),
				// Enemy::wisp(Vector2::new(4, 5)),
			]
		}
	}
}
