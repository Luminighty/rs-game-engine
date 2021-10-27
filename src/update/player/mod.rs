use crate::{game::{self}, input::{InputSystem}, utils::pathfinder};


mod mouse_state;

pub fn update(app: &mut game::Application, game: &mut game::Game, input: &InputSystem) {
	game.player.position.update();
	if !game.player.position.has_target() {
		mouse_state::update_mouse_state(game, input);
	}
}


fn generate_movement(game: &mut game::Game) {
	let player = &game.player;

	let walls = game.occupied_tiles();
	let result = pathfinder::pathfinder(player.position.game_position(), &walls, player.speed as usize);

	game.pathfinder = Some(result);
}