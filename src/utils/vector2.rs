
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
		Self::new(-1, 0)
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


impl From<(i32, i32)> for Vector2 {
	fn from(c: (i32, i32)) -> Self {
		Self::new(c.0, c.1)
	}
}

impl Into<(i32, i32)> for Vector2 {
	fn into(self) -> (i32, i32) {
		(self.x, self.y)
	}
}

#[cfg(test)]
mod test {

	use crate::utils::Vector2;

	#[test]
	pub fn basic_vec() {
		let mut vec = Vector2::new(10, 5);
		assert_eq!(vec.x, 10);
		assert_eq!(vec.y, 5);
		vec.x = 30;
		vec.y = 10;
		assert_eq!(vec.x, 30);
		assert_eq!(vec.y, 10);
	}

	#[test]
	pub fn constants() {
		let v = Vector2::zero();
		assert_eq!(v.x, 0);
		assert_eq!(v.y, 0);
		let v = Vector2::one();
		assert_eq!(v.x, 1);
		assert_eq!(v.y, 1);
		let v = Vector2::right();
		assert_eq!(v.x, 1);
		assert_eq!(v.y, 0);
		let v = Vector2::left();
		assert_eq!(v.x, -1);
		assert_eq!(v.y, 0);
		let v = Vector2::up();
		assert_eq!(v.x, 0);
		assert_eq!(v.y, 1);
		let v = Vector2::down();
		assert_eq!(v.x, 0);
		assert_eq!(v.y, -1);
	}

	#[test]
	pub fn converts() {
		let v: Vector2 = (5, 10).into();
		assert_eq!(v.x,  5);
		assert_eq!(v.y, 10);

		let v = Vector2::new(5, 15);
		let (x, y) = v.into();
		assert_eq!(x, 5);
		assert_eq!(y, 15);

		let v = Vector2::new(5, 10);
		into_test((5, 10), &v);
	}

	fn into_test<T: Into<Vector2>>(v: T, expected: &Vector2) {
		let v: Vector2 = v.into();
		assert_eq!(v.x, expected.x);
		assert_eq!(v.y, expected.y);
	}
}