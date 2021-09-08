pub fn clamp<T: Ord>(min: T, number: T, max: T) -> T {
	std::cmp::max(min, std::cmp::min(number, max))
}

/// Wraps a value, while keeping it between two bounds inclusive
#[derive(Clone, Copy)]
pub struct Clamped<T: Ord + Clone + Copy + Sized> {
	value: T,
	max: T,
	min: T,
}

impl<T: Ord + Clone + Copy + Sized> Clamped<T> {

	pub fn new(min: T, value: T, max: T) -> Self {
		Self {
			min, value, max
		}
	}

	/// Setter for the value
	pub fn set(&mut self, other: T) {
		self.set_unsafe(clamp(self.min, other, self.max));
	}

	/// Getter for the value
	pub fn get(&self) -> T {
		self.value
	}

	/// Sets the maximum, while keeping the clamped restraint 
	pub fn set_max(&mut self, other: T) {
		if other < self.value {
			self.value = other;
		}
		self.set_max_unsafe(other);
	}

	/// Getter for the max
	pub fn get_max(&self) -> T {
		self.max
	}

	/// Sets the minimum, while keeping the clamped restraint 
	pub fn set_min(&mut self, other: T) {
		if other > self.value {
			self.value = other;
		}
		self.set_min_unsafe(other);
	}

	/// Getter for the min
	pub fn get_min(&self) -> T {
		self.min
	}

	/// Sets the value, without keeping the restraint
	pub fn set_unsafe(&mut self, other: T) {
		self.value = other;
	}

	/// Sets the minimum, without keeping the restraint
	pub fn set_min_unsafe(&mut self, other: T) {
		self.min = other;
	}	

	/// Sets the maximum, without keeping the restraint
	pub fn set_max_unsafe(&mut self, other: T) {
		self.max = other;
	}
}



impl<T: Ord + Clone + Copy + Sized + std::ops::Add<Output = T>> std::ops::Add<T> for Clamped<T> {
    type Output = Clamped<T>;

    fn add(mut self, rhs: T) -> Self::Output {
		let value = self.get() + rhs;
		self.set(value);
		self
	}
}

impl<T: Ord + Clone + Copy + Sized + std::ops::AddAssign<> + std::ops::Add<Output = T>> std::ops::AddAssign<T> for Clamped<T> {

    fn add_assign(&mut self, other: T) {
        let value = self.get() + other;
		self.set(value);
    }
}

impl<T: Ord + Clone + Copy + Sized + std::ops::Sub<Output = T>> std::ops::Sub<T> for Clamped<T> {
    type Output = Clamped<T>;

    fn sub(mut self, rhs: T) -> Self::Output {
		let value = self.get() - rhs;
		self.set(value);
		self
	}
}

impl<T: Ord + Clone + Copy + Sized + std::ops::SubAssign<> + std::ops::Sub<Output = T>> std::ops::SubAssign<T> for Clamped<T> {

    fn sub_assign(&mut self, other: T) {
        let value = self.get() - other;
		self.set(value);
    }
}

impl<T: Ord + Clone + Copy + Sized + std::ops::Mul<Output = T>> std::ops::Mul<T> for Clamped<T> {
    type Output = Clamped<T>;

    fn mul(mut self, rhs: T) -> Self::Output {
		let value = self.get() * rhs;
		self.set(value);
		self
	}
}

impl<T: Ord + Clone + Copy + Sized + std::ops::MulAssign<> + std::ops::Mul<Output = T>> std::ops::MulAssign<T> for Clamped<T> {

    fn mul_assign(&mut self, other: T) {
        let value = self.get() * other;
		self.set(value);
    }
}

impl<T: Ord + Clone + Copy + Sized + std::ops::Div<Output = T>> std::ops::Div<T> for Clamped<T> {
    type Output = Clamped<T>;

    fn div(mut self, rhs: T) -> Self::Output {
		let value = self.get() / rhs;
		self.set(value);
		self
	}
}

impl<T: Ord + Clone + Copy + Sized + std::ops::DivAssign<> + std::ops::Div<Output = T>> std::ops::DivAssign<T> for Clamped<T> {

    fn div_assign(&mut self, other: T) {
        let value = self.get() / other;
		self.set(value);
    }
}


#[cfg(test)]
mod tests {
	use crate::utils::clamp::*;
	
	#[test]
	pub fn clamp_test() {
		assert_eq!(clamp(0, 5, 10), 5);
		assert_eq!(clamp(0, 15, 10), 10);
		assert_eq!(clamp(4, 0, 10), 4);
		assert_eq!(clamp(5, 5, 10), 5);
		assert_eq!(clamp(5, 10, 10), 10);
	}

	#[test]
	pub fn clampable_test() {
		let mut clamped = Clamped::new(0, 5, 10);
		assert_eq!(clamped.get(), 5);
		assert_eq!(clamped.get_max(), 10);
		assert_eq!(clamped.get_min(), 0);
		
		clamped.set(-10);
		assert_eq!(clamped.get(), 0);
		
		clamped.set(20);
		assert_eq!(clamped.get(), 10);

		clamped.set_max(8);
		assert_eq!(clamped.get(), 8);
		assert_eq!(clamped.get_max(), 8);

		clamped.set(2);
		clamped.set_min(5);
		assert_eq!(clamped.get(), 5);
		assert_eq!(clamped.get_min(), 5);
	}

	#[test]
	pub fn clampable_ops() {
		let mut clamped = Clamped::new(0, 5, 10);

		clamped = clamped + 4;
		assert_eq!(clamped.get(), 9);
		clamped = clamped + 4;
		assert_eq!(clamped.get(), 10);

		clamped = clamped - 5;
		assert_eq!(clamped.get(), 5);
		clamped = clamped - 10;
		assert_eq!(clamped.get(), 0);

		clamped.set(1);
		clamped = clamped * 2;
		assert_eq!(clamped.get(), 2);
		clamped = clamped * 3;
		assert_eq!(clamped.get(), 6);
		clamped = clamped * 3;
		assert_eq!(clamped.get(), 10);

		clamped.set_min(4);
		clamped = clamped / 2;
		assert_eq!(clamped.get(), 5);
		clamped = clamped / 2;
		assert_eq!(clamped.get(), 4);
	}

	#[test]
	pub fn clampable_assign_ops() {
		let mut clamped = Clamped::new(0, 5, 10);

		clamped += 4;
		assert_eq!(clamped.get(), 9);
		clamped += 4;
		assert_eq!(clamped.get(), 10);

		clamped -= 5;
		assert_eq!(clamped.get(), 5);
		clamped -= 10;
		assert_eq!(clamped.get(), 0);

		clamped.set(1);
		clamped *= 2;
		assert_eq!(clamped.get(), 2);
		clamped *= 3;
		assert_eq!(clamped.get(), 6);
		clamped *= 3;
		assert_eq!(clamped.get(), 10);

		clamped.set_min(4);
		clamped /= 2;
		assert_eq!(clamped.get(), 5);
		clamped /= 2;
		assert_eq!(clamped.get(), 4);
	}
}