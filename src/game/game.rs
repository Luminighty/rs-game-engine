use crate::{game::map::Map, utils::{Matrix, Vector2, pathfinder::{Path, PathfinderResult}}};

use super::{actor::{Enemy, Player}, map::Tile};

pub struct Game {
	pub map: Map,
	pub player: Player,
	pub enemies: Vec<Enemy>,

	pub animation_step: usize,

	pub pathfinder: Option<PathfinderResult>,
	pub path: Option<Path>,
}


impl Game {
	pub fn occupied_tiles(&self) -> Matrix<bool> {
		let size = self.map.get_size();
		let mut tiles = Matrix::new(size.x as usize, size.y as usize, false);

		for (pos, _) in self.map.walls.iter() {
			tiles[[pos.x as usize, pos.y as usize]] = true;
		}

		for i in 0..self.map.tiles.len() {
			if self.map.tiles[i] == Tile::Void {
				tiles[[i % size.x as usize, i / size.x as usize]] = true;
			}
		}

		for enemy in &self.enemies {
			let (x, y) = enemy.position.into();
			tiles[[x as usize, y as usize]] = true;
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
				Enemy::bat(Vector2::new(4, 9)),
				Enemy::bat(Vector2::new(3, 8)),
				Enemy::wisp(Vector2::new(4, 3)),
			],
			pathfinder: None,
			path: None,
		}
	}
}
