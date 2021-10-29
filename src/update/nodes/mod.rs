use crate::input::{InputSystem};
use crate::game;

mod movement;
mod attack;

pub fn update(app: &mut game::Application, game: &mut game::Game, input: &InputSystem) {
	movement::update(app, game, input);
	attack::update(app, game, input);
}