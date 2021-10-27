use std::collections::{HashMap};
use crate::utils::pathfinder::PathNode;

pub use self::movement::MoveNode;

mod movement;

pub struct Nodes {
	pub nodes: HashMap<(u32, u32), Node>,
	pub hovered: Option<((u32, u32), Vec<PathNode>)>,
}

impl Nodes {
	pub fn new() -> Self {
		Self {
			nodes: HashMap::new(),
			hovered: None,
		}
	}

	pub fn add(&mut self, node: Node) {
		self.nodes.insert(node.position, node);
	}

	pub fn get(&self, k: &(u32, u32)) -> Option<&Node> {
		self.nodes.get(k)
	}
}

pub struct Node {
	pub kind: NodeKind,
	pub position: (u32, u32),
}

impl Node {
	pub fn new(position: (u32, u32)) -> Self {
		Self {position, kind: NodeKind::None}
	}

	pub fn movement(position: (u32, u32)) -> Self {
		Self {
			position: position,
			kind: NodeKind::Move,
		}
	}

	pub fn attack(position: (u32, u32)) -> Self {
		Self {
			position,
			kind: NodeKind::Attack,
		}
	}
}

pub enum NodeKind {
	None,
	Move,
	Attack,
}