use crate::{game, utils::{PrQueue, Vector2}};


pub fn pathfinder(game: &game::Game, x: i32, y: i32, max_distance: u32) -> PathfinderResult {
	let mut tiles = Vec::new();
	let tilemap = game.occupied_tiles();
	let mut queue = PrQueue::min(&Tile::distance);
	
	queue.add(Tile {x, y, dist: 0});
	while let Some(tile) = queue.pop() {
		if tile.dist >= max_distance {
			continue;
		}
		try_add_tile(&tiles, &mut queue, &tilemap, tile.x + 1, tile.y + 0, game.map.get_size(), tile.dist + 1);
		try_add_tile(&tiles, &mut queue, &tilemap, tile.x + 0, tile.y + 1, game.map.get_size(), tile.dist + 1);
		try_add_tile(&tiles, &mut queue, &tilemap, tile.x - 1, tile.y - 0, game.map.get_size(), tile.dist + 1);
		try_add_tile(&tiles, &mut queue, &tilemap, tile.x - 0, tile.y - 1, game.map.get_size(), tile.dist + 1);
		tiles.push(tile);
	}

	PathfinderResult {tiles}
}

fn _print_tilemap(tilemap: &Vec<bool>, size: Vector2) {
	for y in 0..size.y {
		for x in 0..size.x {
			print!("{}", if tilemap[(x + y * size.x) as usize] { "#" } else { " " });
		}
		println!("");
	}
}

fn try_add_tile(tiles: &Vec<Tile>, queue: &mut PrQueue<Tile>, tilemap: &Vec<bool>, x: i32, y: i32, map_size: Vector2, distance: u32) {
	if tilemap[(x + y * map_size.x) as usize] { 
		return;
	}
	for tile in tiles.iter() {
		if tile.x == x && tile.y == y {
			return;
		}
	}

	queue.add(Tile::new(x, y, distance));
}

#[derive(Debug)]
pub struct PathfinderResult {
	tiles: Vec<Tile>,
}

impl PathfinderResult {

	pub fn path(&self, x: i32, y: i32) -> Option<Vec<PathNode>> {
		for tile in self.tiles.iter() {
			if tile.x == x && tile.y == y {
				return Some(self.build_path(tile));
			}
		}
		None
	}

	fn build_path(&self, tile: &Tile) -> Vec<PathNode> {
		let node = PathNode {x: tile.x, y: tile.y};
		if tile.distance() == 0 {
			vec![node]
		} else {
			for other in self.tiles.iter() {
				let dist = (other.x - tile.x).abs() + (other.y - tile.y).abs();
				if other.dist == tile.dist - 1 && dist == 1 {
					let mut v = self.build_path(other);
					v.push(node);
					return v;
				}
			}
			panic!("Couldn't find the next node! {:?}", self);
		}
	}

	pub fn nodes(&self) -> Vec<(i32, i32)> {
		self.tiles.iter().map(|tile| (tile.x, tile.y)).collect()
	}
}

#[derive(Debug)]
pub struct PathNode {
	pub x: i32,	
	pub y: i32,
}

#[derive(Debug)]
struct Tile {
	pub x: i32,
	pub y: i32,
	dist: u32,
}

impl Tile {
	fn new(x: i32, y: i32, distance: u32) -> Self {
		Self {x, y, dist: distance}
	}

	pub fn distance(&self) -> usize {
		self.dist as usize
	}
}