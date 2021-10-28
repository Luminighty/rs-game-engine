extern crate sdl2;

mod input_system;
mod button;
mod mouse;

pub use input_system::InputSystem;
pub use mouse::MouseButton;
pub use button::ButtonState;
use sdl2::{rect::Rect, video::FullscreenType};

use crate::render::SdlWrapper;
use crate::game::Application;

use sdl2::event::{Event, WindowEvent};

pub fn get_user_input(input: &mut InputSystem, sdl: &mut SdlWrapper, app: &mut Application) {
	input.update();
	for event in sdl.events.poll_iter() {
		input.push_event(event.clone());
		match event {
			Event::Window {win_event, ..} => match win_event {
				WindowEvent::Resized(w, h) => {
					if sdl.canvas.window().fullscreen_state() != FullscreenType::True {
						sdl.meta.windowed_size = (w as u32, h as u32);
					}
				},
				_ => (),
			}
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
				let viewport = sdl.meta.viewport.unwrap_or(Rect::new(0,0,0,0));
				let upscale = sdl.meta.upscale;
				input.mouse.x = (x - viewport.x) / upscale as i32;
				input.mouse.y = (y - viewport.y) / upscale as i32;
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