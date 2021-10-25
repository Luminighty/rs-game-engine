use crate::utils::clamp::Clamped;

#[derive(Clone, Copy, Debug)]
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

	pub fn hp(&self) -> Clamped<u32> {
		match *self {
			Kind::Bat => Clamped::new(0, 10, 10),
			_         => Clamped::new(0, 10, 10),
		}
	}
}