use std::{iter};

use crate::utils::{Rect, Vector2};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tile {
	Void,
	Ground,
	Tile(u8),
	Grass(u8),
	Stick(u8),
	Flower(u8),
}

type Wall = u8;

pub struct Map {
	width: usize,
	height: usize,
	pub tiles: Vec<Tile>,
	pub walls: Vec<(Vector2, Wall)>,
}

impl Map {
	pub fn new(width: usize, height: usize) -> Self {
		let tiles = iter::repeat_with(|| Tile::Void).take(width * height).collect();
		Self {
			width, height,
			tiles, walls: vec![]
		}
	}

	pub fn set_tile(&mut self, x: i32, y: i32, tile: Tile) {
		let index = self.tile_index(x, y);
		self.tiles[index] = tile;
	}

	pub fn square(mut self, rect: Rect, tile: Tile) -> Self {
		for x in rect.x..(rect.x + rect.width as i32) {
			for y in rect.y..(rect.y + rect.height as i32) {
				self.set_tile(x, y, tile)
			}
		}
		self
	}

	pub fn wall(mut self, position: Vector2, wall: Wall) -> Self {
		self.walls.push((position, wall));
		self
	}

	pub fn get_size(&self) -> Vector2 {
		Vector2::new(self.width as i32, self.height as i32)
	}

	pub fn len(&self) -> usize {
		self.tiles.len()
	}

	pub fn get_index(&self, index: usize) -> Option<&Tile> {
		self.tiles.get(index)
	}
}

impl Map {
	fn tile_index(&self, x: i32, y: i32) -> usize {
		x as usize + self.width * y as usize
	}
}

impl Map {
	pub fn example_map() -> Self {
		Self::new(13, 12)
			.square(Rect::new().offset(2, 2).size(5, 4), Tile::Ground)
			.square(Rect::new().offset(2, 6).size(9, 5), Tile::Ground)
			.wall(Vector2::new(3, 3), 0)
			.wall(Vector2::new(5, 3), 0)
			.wall(Vector2::new(3, 5), 1)
			.wall(Vector2::new(5, 5), 3)
	}
}