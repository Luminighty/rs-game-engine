use crate::utils::Vector2;
use crate::utils::clamp::Clamped;
use super::MouseState;


pub struct Player {
	pub position: Vector2,
	pub hp: Clamped<u32>,
	pub mouse_state: MouseState,
}

impl Player {
	
	pub fn new(position: Vector2) -> Self {
		Self {
			position,
			hp: Clamped::new(0, 10, 10),
			mouse_state: MouseState::Idle,
		}
	}
}