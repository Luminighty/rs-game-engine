use crate::utils::{Vector2, nd_iter};

pub type AttackNodes = Vec<(Vector2, AttackNode)>;

#[derive(Clone, Debug)]
pub enum AttackNode {
	Empty,
	Enemy(usize),
	Origin,
}

impl AttackNode {
	pub fn circle<Vec2: Into<Vector2>>(origin: Vec2, range: usize, targets: Vec<Vector2>) -> AttackNodes {
		let origin: Vector2 = origin.into();
		AttackNode::square(origin, range, targets).iter()
			.filter(|(pos, _)| {
				let delta = origin.delta(pos);
				delta.x + delta.y <= range as i32
			}).cloned().collect()
	}

	pub fn square<Vec2: Into<Vector2>>(origin: Vec2, range: usize, targets: Vec<Vector2>) -> AttackNodes {
		let origin: Vector2 = origin.into();
		let range = range as i32;

		let min = (origin.x - range    , origin.y - range    );
		let max = (origin.x + range + 1, origin.y + range + 1);

		let to_node = |v: Vector2| {
			if origin == v {
				return  (v, AttackNode::Origin);
			}
			match targets.iter().position(|p| *p == v) {
				Some(index) => (v, AttackNode::Enemy(index)),
				None => (v, AttackNode::Empty),
			}
		};

		nd_iter::range_2d(min, max, (1, 1))
			.map(|v| Vector2::new(v[0], v[1]))
			.map(to_node).collect()
	}
}