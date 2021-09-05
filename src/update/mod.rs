use crate::{game, input::{InputSystem}};

pub fn update(app: &mut game::Application, game: &mut game::Game, input: &InputSystem) {
	game.frame += 1;
}