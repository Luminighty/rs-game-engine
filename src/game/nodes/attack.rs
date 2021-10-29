use std::{iter::FromIterator, ops::Deref, slice::Iter};

use crate::utils::Vector2;

pub type AttackNodes = Vec<((i32, i32), AttackNode)>;

#[derive(Clone, Debug)]
pub enum AttackNode {
	Empty,
	Enemy(usize),
}

impl AttackNode {
	pub fn circle<Vec2: Into<Vector2>>(origin: Vec2, range: usize, targets: Vec<Vector2>) -> AttackNodes {
		let origin: Vector2 = origin.into();
		AttackNode::square(origin, range, targets).iter()
			.filter(|(pos, node)| {
				let pos: Vector2 = (*pos).into();
				let delta = origin.delta(&pos);
				delta.x + delta.y <= range as i32
			}).cloned().collect()
	}

	pub fn square<Vec2: Into<Vector2>>(origin: Vec2, range: usize, targets: Vec<Vector2>) -> AttackNodes {
		let origin: Vector2 = origin.into();
		let range = range as i32;

		let x_iter = (origin.x-range)..(origin.x+range+1);
		x_iter.map(|x| (x, (origin.y-range)..(origin.y+range+1)))
			.flat_map(|(x, y_iter)| y_iter.map(move |y| (x, y)))
			.map(|(x, y)| match targets.iter().position(|pos| *pos == (x, y).into()) {
				Some(index) => ((x, y), AttackNode::Enemy(index)),
				None => ((x, y), AttackNode::Empty),
			}).collect()
	}
}