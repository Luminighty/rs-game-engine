use crate::{game::{self, actor::MouseState}, input::{ButtonState, InputSystem, MouseButton}, update::get_actor_rect};

use super::generate_movement;


pub fn update_mouse_state(game: &mut game::Game, input: &InputSystem) {
	let player_rect = get_actor_rect(&game.player.position);
	let mouse_pos = input.mouse.position();
	let contains = player_rect.contains(&mouse_pos);
	let mouse_state = input.mouse.get(MouseButton::Left);

	match (contains, mouse_state) {
		(true, ButtonState::Released)	=> player_clicked(game),
		(true, _)                    	=> player_hover(game),
		(false, _)						=> player_idle(game),
		_ => (),
	}
}

fn player_idle(game: &mut game::Game) {
	game.player.mouse_state = MouseState::Idle;
}

fn player_clicked(game: &mut game::Game) {
	if game.player.mouse_state == MouseState::Selected {
		// Click on player while selected
		game.player.mouse_state = MouseState::Idle;

	} else {
		game.player.mouse_state = MouseState::Selected;
		generate_movement(game);
	}
	
}

fn player_hover(game: &mut game::Game) {
	if game.player.mouse_state != MouseState::Selected {
		game.player.mouse_state = MouseState::Hover;
	}
}
