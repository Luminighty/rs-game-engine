pub struct NdRangeIter<T> {
	/// Stores each dimension as
	/// (min, max, curr, step)
	data: Vec<(T, T, T, T)>,
}

impl<T: Clone + Copy + Ord + Sized + std::ops::Add<Output = T>> Iterator for NdRangeIter<T> {
	type Item = Vec<T>;

	fn next(&mut self) -> Option<Self::Item> {
		let mut advance = true;
		let currents: Vec<T> = self.data.iter().map(|(_, _, c, _)| *c).collect();
		let max: Vec<T> = self.data.iter().map(|(_, max, _, _)| *max).collect();
		if currents.last() == max.last() {
			return None;
		}

		let len = self.data.len();
		for i in 0..len {
			let (min, max, curr, step) = &mut self.data[i];

			if !advance {
				continue;
			}
			let new = *curr + *step;
			*curr = new;
			if curr >= max && i != len-1 {
				*curr = *min
			} else {
				advance = false;
			}
		}
		Some(currents)
	}
}

pub fn range<T: Clone + Ord + Sized + std::ops::Add<Output = T>>(min: T, max: T, step: T) -> NdRangeIter<T> {
	let curr = min.clone();
	NdRangeIter {
		data: vec![(min, max, curr, step)]
	}
}

pub fn range_2d<T: Clone + Ord + Sized + std::ops::Add<Output = T>>(min: (T, T), max: (T, T), step: (T, T)) -> NdRangeIter<T> {
	let curr = min.clone();
	NdRangeIter {
		data: vec![(min.0, max.0, curr.0, step.0), (min.1, max.1, curr.1, step.1)]
	}
}

#[cfg(test)]
mod test {
    use super::{range, range_2d};


	#[test]
	pub fn range_test() {
		let mut expected = -10;
		for i in range(-10, 10, 1) {
			assert_eq!(i.len(), 1);
			assert_eq!(i[0], expected);
			expected += 1;
		}

	}
	#[test]
	pub fn range2d_test() {
		let mut iter = range_2d((0, 0), (3, 3), (1, 1));
		assert_eq!(iter.next(), Some(vec![0, 0]));
		assert_eq!(iter.next(), Some(vec![1, 0]));
		assert_eq!(iter.next(), Some(vec![2, 0]));
		assert_eq!(iter.next(), Some(vec![0, 1]));
		assert_eq!(iter.next(), Some(vec![1, 1]));
		assert_eq!(iter.next(), Some(vec![2, 1]));
		assert_eq!(iter.next(), Some(vec![0, 2]));
		assert_eq!(iter.next(), Some(vec![1, 2]));
		assert_eq!(iter.next(), Some(vec![2, 2]));
		assert_eq!(iter.next(), None);
	}
}