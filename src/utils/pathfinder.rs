use crate::{utils::{PrQueue, Vector2}};

use super::Matrix;

pub type Path = Vec<Vector2>;

pub fn pathfinder<Position: Into<Vector2>>(start: Position, walls: &Matrix<bool>, max_distance: usize) -> PathfinderResult {
	let mut tiles = Vec::new();
	let mut queue = PrQueue::min(&PathNode::distance);
	let start = start.into();
	queue.add(PathNode::new(start.x, start.y, 0));
	while let Some(tile) = queue.pop() {
		if tile.dist >= max_distance {
			continue;
		}
		try_add_tile(&tiles, &mut queue, &walls, tile.x + 1, tile.y + 0, tile.dist + 1);
		try_add_tile(&tiles, &mut queue, &walls, tile.x + 0, tile.y + 1, tile.dist + 1);
		try_add_tile(&tiles, &mut queue, &walls, tile.x - 1, tile.y - 0, tile.dist + 1);
		try_add_tile(&tiles, &mut queue, &walls, tile.x - 0, tile.y - 1, tile.dist + 1);
		tiles.push(tile);
	}

	PathfinderResult::new(tiles)
}

fn try_add_tile(tiles: &Vec<PathNode>, queue: &mut PrQueue<PathNode>, tilemap: &Matrix<bool>, x: i32, y: i32, distance: usize) {
	if tilemap[[x as usize, y as usize]] { 
		return;
	}
	for tile in tiles.iter() {
		if tile.x == x && tile.y == y {
			return;
		}
	}
	let mut node = PathNode::new(x,y,distance);
	node.prev = Some(tiles.len());
	queue.add(node);
}

#[derive(Debug)]
pub struct PathfinderResult {
	pub nodes: Vec<PathNode>,
}

impl PathfinderResult {
	fn new(nodes: Vec<PathNode>) -> Self {
		Self {
			nodes
		}
	}

	pub fn path(&self, x: i32, y: i32) -> Option<Path> {
		if let Some(mut index) = self.nodes.iter().position(|node| node.x == x && node.y == y) {
			let mut v = Vec::new();
			let node = &self.nodes[index];
			v.push(Vector2::new(node.x, node.y));
			while let Some(prev) = self.nodes[index].prev {
				let node = &self.nodes[prev];
				v.push(Vector2::new(node.x, node.y));
				index = prev;
			}
			Some(v)
		} else {
			None
		}
	}
}

#[derive(Debug)]
pub struct PathNode {
	pub x: i32,
	pub y: i32,
	pub prev: Option<usize>,
	dist: usize,
}

impl PathNode {
	fn new(x: i32, y: i32, dist: usize) -> Self {
		Self {
			x, y, prev: None, dist
		}
	}
	fn distance(&self) -> usize {
		self.dist
	}
	pub fn position(&self) -> (i32, i32) {
		(self.x, self.y)
	}
}
