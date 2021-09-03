use crate::{game, input::InputSystem};

pub fn update(app: &mut game::Application, game: &mut game::Game, input: &InputSystem) {
	game.frame += 1;

	if input.quit.is_down() {
		app.is_running = false;
	}
}