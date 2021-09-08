use crate::{game::map::Map, utils::Vector2};

use super::actor::{Enemy, Player};

pub struct Game {
	pub map: Map,
	pub player: Player,
	pub enemies: Vec<Enemy>,

	pub animation_step: usize,
}


impl Game {
	pub fn new() -> Self {
		Self {
			animation_step: 30,
			map: Map::example_map(),
			player: Player::new(Vector2::new(8, 8)),
			enemies: vec![
				Enemy::bat(Vector2::new(3, 4)),
				Enemy::bat(Vector2::new(5, 4)),
				Enemy::wisp(Vector2::new(4, 5)),
			]
		}
	}
}