use crate::utils::Vector2;

pub struct Enemy {
	pub position: Vector2,
	pub kind: Kind,
}

pub enum Kind {
	Bat,
	Ghost,
	Wisp,
	Zombie
}

impl Kind {
	pub fn offset(&self) -> u8 {
		match self {
			&Kind::Bat => 0,
			&Kind::Ghost => 1,
			&Kind::Wisp => 2,
			&Kind::Zombie => 3,
		}
	} 
}

impl Enemy {

	pub fn new(position: Vector2, kind: Kind) -> Self {
		Self { position, kind }
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