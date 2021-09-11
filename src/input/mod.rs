extern crate sdl2;

mod input_system;
mod button;
mod mouse;

pub use input_system::InputSystem;
pub use mouse::MouseButton;

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
			Event::MouseMotion {x,y, ..} => {
				input.mouse.x = x / app.upscale as i32;
				input.mouse.y = y / app.upscale as i32;
			}
			Event::MouseButtonDown {mouse_btn, ..} => {
				if let Some(button) = mouse::MouseButton::from_sdl2(mouse_btn) {
					input.mouse.press(button);
				} else {
					println!("WARNING: Unknown button pressed");
				}
			}
			Event::MouseButtonUp {mouse_btn, ..} => {
				if let Some(button) = mouse::MouseButton::from_sdl2(mouse_btn) {
					input.mouse.release(button);
				} else {
					println!("WARNING: Unknown button released");
				}
			}
			_ => {}
		}
	}
}