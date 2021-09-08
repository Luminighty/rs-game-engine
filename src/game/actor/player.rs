use crate::utils::Vector2;
use crate::utils::clamp::Clamped;


pub struct Player {
	pub position: Vector2,
	pub hp: Clamped<u32>,
}

impl Player {
	
	pub fn new(position: Vector2) -> Self {
		Self {
			position,
			hp: Clamped::new(0, 10, 10),
		}
	}
}