use std::slice::Iter;

use crate::utils::EventSystem;

use super::AppEvent;

pub struct Application {
	pub is_running: bool,
	pub frame: usize,

	_events: EventSystem<AppEvent>,
}


impl Application {
	pub fn new() -> Self {
		Self {
			is_running: true,
			frame: 0,
			_events: EventSystem::new(),
		}
	}

	pub fn quit(&mut self) {
		self.is_running = false;
	}

	pub fn push_event(&mut self, event: AppEvent) {
		self._events.push_event(event);
	}

	pub fn events(&self) -> Iter<AppEvent> {
		self._events.iter()
	}

	pub fn clear_events(&mut self) {
		self._events.clear();
	}
}