use crate::game::actor::MouseState;
use crate::input::{ButtonState, InputSystem, MouseButton};
use crate::game;
use crate::update::get_actor_rect;

pub fn update(app: &mut game::Application, game: &mut game::Game, input: &InputSystem) {
	let mut remove_pathfinder = false;
	let mut has_hover = false;
	if let Some(pathfinder) = &game.pathfinder {
		for i in 0..pathfinder.nodes.len() {

			let node = &pathfinder.nodes[i];

			let contains = get_actor_rect(node.position()).contains(&input.mouse.position());
			let mouse_state = input.mouse.get(MouseButton::Left);

			has_hover = has_hover || contains;
			match (contains, mouse_state) {
				(true, ButtonState::Released) => {
					// press
					if let Some(path) = &game.path {
						for target in path.iter().rev() {
							game.player.position.add_target(*target);
						}
						game.path = None;
						remove_pathfinder = true;
						game.player.mouse_state = MouseState::Idle;
						break;
					}
				},
				(true, _) => {
					// hover
					let is_new_path = game.path.iter().all(|path| path.len() == 0 || path[0].x != node.x || path[0].y != node.y);
					if is_new_path {
						game.path = pathfinder.path(node.x, node.y);
					}
				},
				_ => (),
			}
		}
	}
	if remove_pathfinder {
		game.pathfinder = None;
	}
	if !has_hover {
		game.path = None;
	}
}