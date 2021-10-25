use crate::{game::{self}, input::{InputSystem}, utils};

pub mod pathfinder;
pub mod player;

pub fn update(app: &mut game::Application, game: &mut game::Game, input: &InputSystem) {
	app.frame += 1;

	player::player_update(app, game, input);

}

pub fn get_actor_rect(vector: &utils::Vector2) -> utils::Rect {
	utils::Rect::new().offset(vector.x * 16, vector.y * 16).size(16, 16)
}