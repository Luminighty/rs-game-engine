extern crate sdl2;


use sdl2::{render::TextureCreator, video::WindowContext};

use crate::config;


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
			.build()
			.unwrap();

		let canvas = window.into_canvas().build().unwrap();

		let events = sdl.event_pump().unwrap();

		Self {
			canvas, events
		}
	}

	pub fn texture_creator(&self) -> TextureCreator<WindowContext> {
		self.canvas.texture_creator()
	}

}