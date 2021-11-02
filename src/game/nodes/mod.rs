use crate::utils::{Vector2, pathfinder::{Path, PathfinderResult}};

use self::attack::AttackNodes;

pub mod attack;

pub struct Nodes {
	pub attack: Option<AttackNodes>,
	pub attack_hover: Option<Vector2>,
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