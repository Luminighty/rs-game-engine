use crate::utils::{DelayedVector2, Vector2};
use crate::utils::clamp::Clamped;
use super::MouseState;

pub struct Player {
	pub position: DelayedVector2,
	pub hp: Clamped<u32>,
	pub mouse_state: MouseState,
	pub speed: u32,
}

impl Player {
	pub fn new(position: Vector2) -> Self {
		Self {
			position: DelayedVector2::new(position, 1), 
			hp: Clamped::new(0, 10, 10),
			mouse_state: MouseState::Idle,
			speed: 4,
		}
	}
}