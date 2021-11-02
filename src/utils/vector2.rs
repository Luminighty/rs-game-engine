use std::{ops::{Add, Div, Mul, Rem, Sub}};


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
	pub x: i32,
	pub y: i32,
}

#[allow(dead_code)]
impl Vector2 {
	/// Initializes a new Vector (x, y)
	/// ## Test
	/// ```
	/// let vec = Vector2::new(3, 2);
	/// assert_eq!(vec.x, 3);
	/// assert_eq!(vec.y, 2);
	/// ```
	pub fn new(x: i32, y: i32) -> Self {
		Self { x, y }
	}

	/// Shorthand for __(0, 0)__
	/// ## Test
	/// ```
	/// let zero = Vector2::zero();
	/// assert_eq!(zero, Vector2::new(0, 0));
	/// ```
	pub fn zero() -> Self {
		Self::new(0, 0)
	}

	/// Shorthand for __(1, 1)__
	/// ## Test
	/// ```
	/// let one = Vector2::one();
	/// assert_eq!(one, Vector2::new(1, 1));
	/// ```
	pub fn one() -> Self {
		Self::new(1, 1)
	}

	/// Shorthand for __(-1, 0)__
	/// ## Test
	/// ```
	/// let left = Vector2::left();
	/// assert_eq!(left, Vector2::new(-1, 0));
	/// ```
	pub fn left() -> Self {
		Self::new(-1, 0)
	}

	/// Shorthand for __(1, 0)__
	/// ## Test
	/// ```
	/// let right = Vector2::right();
	/// assert_eq!(right, Vector2::new(1, 0));
	/// ```
	pub fn right() -> Self {
		Self::new(1, 0)
	}

	/// Shorthand for __(0, 1)__
	/// ## Test
	/// ```
	/// let up = Vector2::up();
	/// assert_eq!(up, Vector2::new(0, 1));
	/// ```
	pub fn up() -> Self {
		Self::new(0, 1)
	}

	/// Shorthand for __(0, -1)__
	/// ## Test
	/// ```
	/// let down = Vector2::down();
	/// assert_eq!(down, Vector2::new(0, -1));
	/// ```
	pub fn down() -> Self {
		Self::new(0, -1)
	}

	/// Returns the squared length of this vector using the following formula
	/// `(x * x) + (y * y)`
	/// ## Test
	/// ```
	/// let vec = Vector2::new(2, 3);
	/// assert_eq!(vec.sqr_magnitude(), 13);
	/// ```
	pub fn sqr_magnitude(&self) -> u32 {
		(self.x * self.x + self.y * self.y) as u32
	}

	pub fn move_towards(&self, target: Vector2, max_delta: u32) -> Self {
		let delta = target - *self;
		let mut vec = Vector2::new(0, 0);
		vec.x = if (i32::abs(delta.x) as u32) < max_delta {
			target.x
		} else {
			self.x + i32::signum(delta.x) * max_delta as i32
		};
		vec.y = if (i32::abs(delta.y) as u32) < max_delta {
			target.y
		} else {
			self.y + i32::signum(delta.y) * max_delta as i32
		};
		
		vec
	}

	pub fn abs(&self) -> Self {
		Self {
			x: self.x.abs(),
			y: self.y.abs(),
		}
	}

	pub fn delta(&self, other: &Vector2) -> Self {
		(*self - *other).abs()
	}
}

impl Sub<Vector2> for Vector2 {
	type Output = Vector2;
	fn sub(self, rhs: Vector2) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}	
	}
}

impl Add<Vector2> for Vector2 {
	type Output = Vector2;
	fn add(self, rhs: Vector2) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}	
	}
}

impl Mul<i32> for Vector2 {
	type Output = Vector2;
	fn mul(self, rhs: i32) -> Self::Output {
		Self {
			x: self.x * rhs,
			y: self.y * rhs,
		}
	}
}

impl Div<i32> for Vector2 {
	type Output = Vector2;
	fn div(self, rhs: i32) -> Self::Output {
		Self {
			x: self.x / rhs,
			y: self.y / rhs,
		}
	}
}

impl Rem<i32> for Vector2 {
	type Output = Vector2;
	fn rem(self, rhs: i32) -> Self::Output {
		Self {
			x: self.x % rhs,
			y: self.y % rhs,
		}
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

impl From<(u32, u32)> for Vector2 {
	fn from(c: (u32, u32)) -> Self {
		Self::new(c.0 as i32, c.1 as i32)
	}
}


impl From<&Vector2> for Vector2 {
	fn from(v: &Vector2) -> Self {
		Self::new(v.x, v.y)
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

	#[test]
	fn move_towards() {
		let v = Vector2::new(10, 15);
		assert_eq!(v.move_towards(Vector2::new(10, 10), 1), Vector2::new(10, 14));
		assert_eq!(v.move_towards(Vector2::new(10, 10), 2), Vector2::new(10, 13));
		assert_eq!(v.move_towards(Vector2::new(10, 10), 10), Vector2::new(10, 10));

		let v = Vector2::new(10, 15);
		assert_eq!(v.move_towards(Vector2::new(15, 15), 1), Vector2::new(11, 15));
		assert_eq!(v.move_towards(Vector2::new(15, 15), 2), Vector2::new(12, 15));
		assert_eq!(v.move_towards(Vector2::new(15, 15), 10), Vector2::new(15, 15));

		let v = Vector2::new(15, 15);
		assert_eq!(v.move_towards(Vector2::new(10, 10), 1), Vector2::new(14, 14));
		assert_eq!(v.move_towards(Vector2::new(10, 10), 2), Vector2::new(13, 13));
		assert_eq!(v.move_towards(Vector2::new(10, 10), 10), Vector2::new(10, 10));

		let v = Vector2::new(30, 15);
		assert_eq!(v.move_towards(Vector2::new(10, 10), 10), Vector2::new(20, 10));

	}
}