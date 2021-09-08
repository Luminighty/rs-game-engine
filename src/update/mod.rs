use crate::{game, input::{InputSystem}};

pub fn update(app: &mut game::Application, _game: &mut game::Game, _input: &InputSystem) {
	app.frame += 1;


}