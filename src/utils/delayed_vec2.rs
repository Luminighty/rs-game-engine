use crate::{UNIT, utils::Vector2};

pub struct DelayedVector2 {
	targets: Vec<Vector2>,
	actual: Vector2,
	offset: Vector2,
	speed: u32,
}

impl DelayedVector2 {
	pub fn new(actual: Vector2, speed: u32) -> Self {
		Self {
			actual, offset: Vector2::zero(),
			speed,
			targets: Vec::new(),
		}
	}

	pub fn update(&mut self) {
		if let Some(target) = self.targets.get(0) {
			let position = (self.render_position() * UNIT) + self.render_offset();
			let new_pos = position.move_towards(*target * UNIT, self.speed);
			
			self.actual = new_pos / UNIT;
			self.offset = new_pos % UNIT;

			if self.actual == *target && self.offset == Vector2::zero() {
				self.targets.remove(0);
			}
		}
	}

	pub fn render_position(&self) -> Vector2 {
		self.actual
	}

	pub fn render_offset(&self) -> Vector2 {
		self.offset
	}

	pub fn game_position(&self) -> Vector2 {
		*self.targets.last().unwrap_or(&self.actual)
	}

	pub fn add_target(&mut self, target: Vector2) {
		self.targets.push(target);
	}

	pub fn has_target(&self) -> bool {
		return self.targets.len() != 0
	}
}

#[cfg(test)]
mod test {
    use crate::utils::Vector2;
    use super::DelayedVector2;

}