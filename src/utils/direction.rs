use std::convert::{TryFrom};

use super::Vector2;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
	Left = 0, Right,
	Up, Down
}

impl Into<Vector2> for Direction {
	fn into(self) -> Vector2 {
		match self {
			Direction::Down => Vector2::down(),
			Direction::Up => Vector2::up(),
			Direction::Left => Vector2::left(),
			Direction::Right => Vector2::right(),
		}
	}
}

impl TryFrom<Vector2> for Direction {
	type Error = ();
	fn try_from(value: Vector2) -> Result<Self, Self::Error> {
		match (value.x, value.y) {
			(1, 0)  => Ok(Direction::Right),
			(-1, 0) => Ok(Direction::Left),
			(0, 1)  => Ok(Direction::Up),
			(0, -1) => Ok(Direction::Down),
			_ => Err(()),
		}
	}
}

#[derive(Debug)]
pub struct DirectionMap {
	map: [bool; 4],
}

impl DirectionMap {
	pub fn new() -> Self {
		Self {
			map: [false, false, false, false],
		}
	}

	pub fn set(&mut self, direction: Direction) {
		self.map[direction as usize] = true;
	}

	pub fn reset(&mut self, direction: Direction) {
		self.map[direction as usize] = false;
	}

	pub fn get(&self, direction: Direction) -> bool {
		self.map[direction as usize]
	}

	pub fn has_left(&self) -> bool {
		self.get(Direction::Left)
	}
	pub fn has_right(&self) -> bool {
		self.get(Direction::Right)
	}
	pub fn has_up(&self) -> bool {
		self.get(Direction::Up)
	}
	pub fn has_down(&self) -> bool {
		self.get(Direction::Down)
	}

	pub fn is_vertical(&self) -> bool {
		self.has_up() && self.has_down()
	}
	pub fn is_horizontal(&self) -> bool {
		self.has_left() && self.has_right()
	}
}