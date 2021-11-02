use sdl2::keyboard::Keycode;

use crate::game::nodes::attack::AttackNode;
use crate::input::{InputSystem};
use crate::game;
use crate::update::get_actor_rect;

pub fn update(app: &mut game::Application, game: &mut game::Game, input: &InputSystem) {

	for event in input.events() {
		match event {
			sdl2::event::Event::KeyUp {keycode, ..} => {
				match keycode {
					Some(Keycode::Num1) => attack_nodes(game),
					Some(Keycode::Num2) => game.nodes.attack = None,
					_ => (),
				}
			},
			_ => (),
		}
	}

	let mouse_pos = input.mouse.position();

	if let Some(attacks) = &game.nodes.attack {
		game.nodes.attack_hover = attacks.iter().filter(|(p, n)| match n {
			&AttackNode::Empty => false,
			&AttackNode::Enemy(_) | &AttackNode::Origin => get_actor_rect(*p).contains(&mouse_pos),
		}).map(|(p, _)| *p).next();
	}
}

fn attack_nodes(game: &mut game::Game) {
	let player = &game.player; 
	let targets = game.enemies.iter().map(|enemy| enemy.position).collect();
	game.nodes.attack = Some(AttackNode::circle(player.position.game_position(), 2, targets));
}