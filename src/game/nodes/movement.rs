pub struct MoveNode {
	pub prev:  (u32, u32),
}

impl MoveNode {
	pub fn new(prev: (u32, u32)) -> Self {
		Self {
			prev
		}
	}
}