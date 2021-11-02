use crate::utils::{Vector2, clamp::Clamped};

pub mod kind;

pub use kind::Kind;

pub struct Enemy {
	pub position: Vector2,
	pub offset: Vector2,
	pub kind: Kind,
	pub hp: Clamped<u32>,
}


impl Enemy {

	pub fn new(position: Vector2, kind: Kind) -> Self {
		Self { position, offset: Vector2::zero(), kind, hp: kind.hp() }
	}

	pub fn bat(position: Vector2) -> Self {
		Enemy::new(position, Kind::Bat)
	}
	
	pub fn ghost(position: Vector2) -> Self {
		Enemy::new(position, Kind::Ghost)
	}
	
	pub fn wisp(position: Vector2) -> Self {
		Enemy::new(position, Kind::Wisp)
	}

	pub fn zombie(position: Vector2) -> Self {
		Enemy::new(position, Kind::Zombie)
	}
}