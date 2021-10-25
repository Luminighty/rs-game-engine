use crate::{game::{self}, input::{InputSystem}, update::pathfinder};


mod mouse_state;

pub fn player_update(app: &mut game::Application, game: &mut game::Game, input: &InputSystem) {
	mouse_state::update_mouse_state(game, input);

}


fn generate_movement(game: &mut game::Game) {
	let player = &game.player;

	let result = pathfinder::pathfinder(game, player.position.x, player.position.y, player.speed);
	game.player.path = Some(result);
}