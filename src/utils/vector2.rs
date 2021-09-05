
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
	pub x: i32,
	pub y: i32,
}

#[allow(dead_code)]
impl Vector2 {
	pub fn new(x: i32, y: i32) -> Self {
		Self { x, y }
	}

	pub fn zero() -> Self {
		Self::new(0, 0)
	}

	pub fn one() -> Self {
		Self::new(1, 1)
	}

	pub fn left() -> Self {
		Self::new(-1, 1)
	}

	pub fn right() -> Self {
		Self::new(1, 0)
	}

	pub fn up() -> Self {
		Self::new(0, 1)
	}

	pub fn down() -> Self {
		Self::new(0, -1)
	}
}