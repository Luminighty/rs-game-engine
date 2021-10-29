use crate::utils::pathfinder::{Path, PathfinderResult};

use self::attack::AttackNodes;

pub mod attack;

pub struct Nodes {
	pub attack: Option<AttackNodes>,
	pub attack_hover: Option<(i32, i32)>,
	pub pathfinder: Option<PathfinderResult>,
	pub path: Option<Path>,
}

impl Nodes {
	pub fn new() -> Self {
		Self {
			path: None,
			pathfinder: None,
			attack: None,
			attack_hover: None,
		}
	}
}