extern crate sdl2;


use sdl2::{image::LoadSurface, render::TextureCreator, surface::Surface, video::{WindowContext}};

use crate::{config, log_err};


pub struct SdlWrapper {
	pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
	pub events: sdl2::EventPump,
}


impl SdlWrapper {
	pub fn init(config: &config::Config) -> Self {
		let sdl = sdl2::init().unwrap();
		let video = sdl.video().unwrap();
		let window = video
			.window(config.window_title.as_str(), config.window_width, config.window_height)
			.position_centered()
			.resizable()
			.build()
			.unwrap();

		let mut canvas = window.into_canvas().build().unwrap();

		let events = sdl.event_pump().unwrap();

		if let Some(icon_path) = &config.icon {
			match Surface::from_file(icon_path) {
				Ok(icon) => canvas.window_mut().set_icon(icon),
				Err(err) => log_err!("Icon: {}", err),
			}
		}


		Self {
			canvas, events
		}
	}

	pub fn texture_creator(&self) -> TextureCreator<WindowContext> {
		self.canvas.texture_creator()
	}

	pub fn set_fullscreen(&mut self, fullscreen: FullscreenType) {
		self.canvas.window_mut().set_fullscreen(fullscreen).unwrap();
	}
}

pub type FullscreenType = sdl2::video::FullscreenType;