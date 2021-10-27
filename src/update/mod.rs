use crate::{UNIT, game::{self}, input::{InputSystem}, utils::{self, Vector2}};

mod player;
mod debug;
mod nodes;

pub fn update(app: &mut game::Application, game: &mut game::Game, input: &InputSystem) {
	app.frame += 1;

	player::update(app, game, input);
	nodes::update(app, game, input);
	debug::update(app, game, input);
}

pub fn get_actor_rect<Vec2: Into<utils::Vector2>>(vector: Vec2) -> utils::Rect {
	let vector: Vector2 = vector.into();
	utils::Rect::new().offset(vector.x * UNIT, vector.y * UNIT).size(UNIT as u32, UNIT as u32)
}