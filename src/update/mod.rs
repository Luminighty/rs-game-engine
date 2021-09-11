use crate::{game::{self, actor::MouseState}, input::{InputSystem}, utils};

pub fn update(app: &mut game::Application, game: &mut game::Game, input: &InputSystem) {
	app.frame += 1;

	let player_rect = get_actor_rect(&game.player.position);
	let mouse_pos = input.mouse.position();
	game.player.mouse_state = MouseState::Idle;
	if player_rect.contains(&mouse_pos) {
		game.player.mouse_state = MouseState::Hover;
	}
}

fn get_actor_rect(vector: &utils::Vector2) -> utils::Rect {
	utils::Rect::new().offset(vector.x * 16, vector.y * 16).size(16, 16)
}