use sdl2::keyboard::Keycode;

use crate::UNIT;
use crate::game::nodes::attack::AttackNode;
use crate::{game, input::InputSystem};
use crate::input::{ButtonState, MouseButton};

pub fn update(app: &mut game::Application, game: &mut game::Game, input: &InputSystem) {

	if input.quit.is_released() {
		app.quit();
	}

	let mouse_btn = input.mouse.get(MouseButton::Right);
	if mouse_btn == ButtonState::Pressed {
		let mouse_pos = input.mouse.position();
		println!("Mouse: ({},{})", mouse_pos.x / UNIT, mouse_pos.y / UNIT);
	}

}
