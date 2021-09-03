extern crate sdl2;

mod input_system;
mod button;

pub use input_system::InputSystem;

use crate::render::SdlWrapper;
use crate::game::Application;

use sdl2::event::Event;



pub fn get_user_input(input: &mut InputSystem, sdl: &mut SdlWrapper, app: &mut Application) {
	input.update();
	for event in sdl.events.poll_iter() {
		match event {
			Event::Quit {..} => {
				app.is_running = false;
			}
			Event::KeyDown {keycode, repeat, ..} => {
				if repeat { continue; }
				if let Some(keycode) = keycode {
					input.press_key(keycode);
				}
			}
			Event::KeyUp {keycode, repeat, ..} => {
				if repeat { continue; }
				if let Some(keycode) = keycode {
					input.release_key(keycode);
				}
			}
			_ => {}
		}
	}
}