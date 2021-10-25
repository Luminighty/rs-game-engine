pub struct PrQueue<'a, T> {
	data: Vec<T>,
	values: Vec<usize>,
	value_function: &'a dyn Fn(&T) -> usize,
	queue_type: PrQueueType,
}

enum PrQueueType {
	/// The min element will always be on top
	/// When adding an element, it will always be the above the same values
	MinQueue,
	/// The max element will always be on top
	/// When adding an element, it will always be the above the same values
	MaxQueue,
	/// The min element will always be on top
	/// When adding an element, it will always be the below the same values
	MinEqQueue,
	/// The max element will always be on top
	/// When adding an element, it will always be the below the same values
	MaxEqQueue,
}

impl<'a, T> PrQueue<'a, T> {

	fn new(value_function: &'a dyn Fn(&T) -> usize, queue_type: PrQueueType) -> Self {
		Self {
			data: Vec::new(),
			values: Vec::new(),
			value_function,
			queue_type,
		}
	}

	/// The max element will always be on top
	/// When adding an element, it will always be the above the same values
	pub fn max(value_function: &'a dyn Fn(&T) -> usize) -> Self {
		Self::new(value_function, PrQueueType::MaxQueue)
	}

	/// The max element will always be on top
	/// When adding an element, it will always be the below the same values
	pub fn max_eq(value_function: &'a dyn Fn(&T) -> usize) -> Self {
		Self::new(value_function, PrQueueType::MaxEqQueue)
	}

	/// The min element will always be on top
	/// When adding an element, it will always be the above the same values
	pub fn min(value_function: &'a dyn Fn(&T) -> usize) -> Self {
		Self::new(value_function, PrQueueType::MinQueue)
	}

	/// The min element will always be on top
	/// When adding an element, it will always be the below the same values
	pub fn min_eq(value_function: &'a dyn Fn(&T) -> usize) -> Self {
		Self::new(value_function, PrQueueType::MinEqQueue)
	}

	pub fn add(&mut self, element: T) {
		let mut i = 0;
		let value = (self.value_function)(&element);

		while i < self.values.len() && self.queue_type.is_below(value, self.values[i]) {
			i += 1;
		}

		if i < self.values.len() {
			self.values.insert(i, value);
			self.data.insert(i, element);
		} else {
			self.values.push(value);
			self.data.push(element);
		}
	}

	pub fn pop(&mut self) -> Option<T> {
		if self.is_empty() {
			None
		} else {
			self.values.remove(0);
			Some(self.data.remove(0))
		}
	}

	pub fn top(&self) -> Option<&T> {
		if self.is_empty() {
			None
		} else {
			Some(&self.data[0])
		}
	}

	pub fn len(&self) -> usize {
		self.data.len()
	}

	pub fn is_empty(&self) -> bool {
		self.data.is_empty()
	}
}

impl PrQueueType {
	pub fn is_below(&self, a: usize, b: usize) -> bool {
		match *self {
			PrQueueType::MinQueue => a > b,
			PrQueueType::MinEqQueue => a >= b,
			PrQueueType::MaxQueue => a < b,
			PrQueueType::MaxEqQueue => a <= b,
		}
	}
}


#[cfg(test)]
mod test {
	use crate::utils::prqueue::*;

	#[derive(PartialEq, Eq, Debug)]
	struct Pair {
		pub id: usize,
		value: usize,
	}
	impl Pair {
		pub fn new(id: usize, value: usize) -> Self {
			Self {id, value}
		}

		pub fn value(&self) -> usize {
			self.value
		}
	}

	fn usize_identity(a: &usize) -> usize { *a }

	#[test]
	pub fn basic_max() {
		let mut q: PrQueue<usize> = PrQueue::max(&usize_identity);
		q.add(5);
		q.add(20);
		q.add(30);
		q.add(15);
		q.add(15);
		assert_eq!(q.pop(), Some(30));
		assert_eq!(q.pop(), Some(20));
		assert_eq!(q.pop(), Some(15));
		assert_eq!(q.pop(), Some(15));
		assert_eq!(q.pop(), Some(5));
		assert_eq!(q.pop(), None);
	}

	#[test]
	pub fn basic_min() {
		let mut q: PrQueue<usize> = PrQueue::min(&usize_identity);
		q.add(5);
		q.add(20);
		q.add(30);
		q.add(30);
		q.add(15);
		assert_eq!(q.len(), 5);
		assert!(!q.is_empty());
		assert_eq!(q.pop(), Some(5));
		assert_eq!(q.pop(), Some(15));
		assert_eq!(q.pop(), Some(20));
		assert_eq!(q.pop(), Some(30));
		assert_eq!(q.pop(), Some(30));
		assert_eq!(q.pop(), None);
		assert!(q.is_empty());
	}

	#[test]
	pub fn basic_misc() {
		let mut q: PrQueue<usize> = PrQueue::min(&usize_identity);
		assert!(q.is_empty());
		assert_eq!(q.pop(), None);
		assert_eq!(q.top(), None);
		assert_eq!(q.len(), 0);
		q.add(10);
		assert!(!q.is_empty());
		assert_eq!(q.top(), Some(&10));
		assert_eq!(q.len(), 1);
		assert_eq!(q.pop(), Some(10));
		assert_eq!(q.len(), 0);
	}

	#[test]
	pub fn struct_max() {
		let mut q: PrQueue<Pair> = PrQueue::max(&Pair::value);
		q.add(Pair::new(0, 5));
		q.add(Pair::new(1, 5));
		q.add(Pair::new(2, 10));
		q.add(Pair::new(3, 10));
		q.add(Pair::new(4, 20));
		q.add(Pair::new(5, 20));
		assert_eq!(q.pop(), Some(Pair::new(5, 20)));
		assert_eq!(q.pop(), Some(Pair::new(4, 20)));
		assert_eq!(q.pop(), Some(Pair::new(3, 10)));
		assert_eq!(q.pop(), Some(Pair::new(2, 10)));
		assert_eq!(q.pop(), Some(Pair::new(1, 5)));
		assert_eq!(q.pop(), Some(Pair::new(0, 5)));
		assert_eq!(q.pop(), None);
	}

	#[test]
	pub fn struct_max_eq() {
		let mut q: PrQueue<Pair> = PrQueue::max_eq(&Pair::value);
		q.add(Pair::new(0, 5));
		q.add(Pair::new(1, 5));
		q.add(Pair::new(2, 10));
		q.add(Pair::new(3, 10));
		q.add(Pair::new(4, 20));
		q.add(Pair::new(5, 20));
		assert_eq!(q.pop(), Some(Pair::new(4, 20)));
		assert_eq!(q.pop(), Some(Pair::new(5, 20)));
		assert_eq!(q.pop(), Some(Pair::new(2, 10)));
		assert_eq!(q.pop(), Some(Pair::new(3, 10)));
		assert_eq!(q.pop(), Some(Pair::new(0, 5)));
		assert_eq!(q.pop(), Some(Pair::new(1, 5)));
		assert_eq!(q.pop(), None);

	}


	#[test]
	pub fn struct_min() {
		let mut q: PrQueue<Pair> = PrQueue::min(&Pair::value);
		q.add(Pair::new(0, 5));
		q.add(Pair::new(1, 5));
		q.add(Pair::new(2, 10));
		q.add(Pair::new(3, 10));
		q.add(Pair::new(4, 20));
		q.add(Pair::new(5, 20));
		assert_eq!(q.pop(), Some(Pair::new(1, 5)));
		assert_eq!(q.pop(), Some(Pair::new(0, 5)));
		assert_eq!(q.pop(), Some(Pair::new(3, 10)));
		assert_eq!(q.pop(), Some(Pair::new(2, 10)));
		assert_eq!(q.pop(), Some(Pair::new(5, 20)));
		assert_eq!(q.pop(), Some(Pair::new(4, 20)));
		assert_eq!(q.pop(), None);
	}

	#[test]
	pub fn struct_min_eq() {
		let mut q: PrQueue<Pair> = PrQueue::min_eq(&Pair::value);
		q.add(Pair::new(0, 5));
		q.add(Pair::new(1, 5));
		q.add(Pair::new(2, 10));
		q.add(Pair::new(3, 10));
		q.add(Pair::new(4, 20));
		q.add(Pair::new(5, 20));
		assert_eq!(q.pop(), Some(Pair::new(0, 5)));
		assert_eq!(q.pop(), Some(Pair::new(1, 5)));
		assert_eq!(q.pop(), Some(Pair::new(2, 10)));
		assert_eq!(q.pop(), Some(Pair::new(3, 10)));
		assert_eq!(q.pop(), Some(Pair::new(4, 20)));
		assert_eq!(q.pop(), Some(Pair::new(5, 20)));
		assert_eq!(q.pop(), None);

	}

}