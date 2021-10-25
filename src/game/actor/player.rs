use crate::update::pathfinder::PathfinderResult;
use crate::utils::Vector2;
use crate::utils::clamp::Clamped;
use super::MouseState;

pub struct Player {
	pub position: Vector2,
	pub offset: Vector2,
	pub hp: Clamped<u32>,
	pub mouse_state: MouseState,
	pub speed: u32,
	
	pub path: Option<PathfinderResult>,
}

impl Player {
	pub fn new(position: Vector2) -> Self {
		Self {
			position, offset: Vector2::zero(),
			hp: Clamped::new(0, 10, 10),
			mouse_state: MouseState::Idle,
			speed: 4,
			path: None,
		}
	}
}