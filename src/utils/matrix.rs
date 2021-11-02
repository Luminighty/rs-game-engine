
pub struct Matrix<T> {
	data: Vec<T>,
	width: usize,
	height: usize,
}

impl<T: Clone> Matrix<T> {
	pub fn fill(mut self, default: T) -> Self {
		self.data = vec![default; self.width * self.height];
		self
	}

	pub fn new(width: usize, height: usize, value: T) -> Self {
		Self {
			data: vec![value; width * height],
			width, height
		}
	}
}

impl<T> Matrix<T> {
	pub fn get(&self, col: usize, row: usize) -> Option<&T> {
		self.data.get(self.index(col, row))
	}

	fn index(&self, col: usize, row: usize) -> usize {
		row * self.width + col
	}

}

impl<T> std::ops::Index<[usize; 2]> for Matrix<T> {
	type Output = T;
	fn index<'a>(&'a self, [col, row]: [usize; 2]) -> &'a T {
		let index = self.index(col, row);
		&self.data[index]
	}
}

impl<T> std::ops::IndexMut<[usize; 2]> for Matrix<T> {
	fn index_mut<'a>(&'a mut self, [col, row]: [usize; 2]) -> &'a mut T {
		let index = self.index(col, row);
		&mut self.data[index]
	}
}