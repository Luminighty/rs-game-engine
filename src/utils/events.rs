use std::slice::Iter;

pub struct EventSystem<T> {
	events: Vec<T>
}

impl<T> EventSystem<T> {
	pub fn new() -> Self {
		Self {
			events: Vec::new(),
		}
	}

	pub fn push_event(&mut self, event: T) {
		self.events.push(event);
	}

	pub fn iter(&self) -> Iter<T> {
		self.events.iter()
	}

	pub fn clear(&mut self) {
		self.events.clear();
	}
}