extern crate sdl2;


use sdl2::{image::LoadSurface, rect::Rect, render::TextureCreator, surface::Surface, video::{FullscreenType, WindowContext}};
use sdl2::pixels::Color;
use crate::{config, log_err};


pub struct SdlWrapper {
	pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
	pub events: sdl2::EventPump,
	pub meta: RendererMetaData,
}

pub struct RendererMetaData {
	pub upscale: u32,
	pub viewport: Option<Rect>,
	pub clear_color: Color,
	pub windowed_size: (u32, u32),
}

impl RendererMetaData {
	pub fn new(config: &config::Config) -> Self {
		Self {
			upscale: 1,
			viewport: None,
			clear_color: Color::RGB(2, 1, 13),
			windowed_size: config.window_size,
		}
	}
}


impl SdlWrapper {
	pub fn init(config: &config::Config) -> Self {
		let sdl = sdl2::init().unwrap();
		let video = sdl.video().unwrap();
		let window = video
			.window(config.window_title.as_str(), config.window_size.0, config.window_size.1)
			.position_centered()
			.resizable()
			.build()
			.unwrap();

		let mut canvas = window.into_canvas().build().unwrap();

		canvas.window_mut().set_minimum_size(config.window_min_size.0, config.window_min_size.1).unwrap();

		let events = sdl.event_pump().unwrap();



		if let Some(icon_path) = &config.icon {
			match Surface::from_file(icon_path) {
				Ok(icon) => canvas.window_mut().set_icon(icon),
				Err(err) => log_err!("Icon: {}", err),
			}
		}


		Self {
			canvas, events, meta: RendererMetaData::new(config),
		}
	}

	pub fn texture_creator(&self) -> TextureCreator<WindowContext> {
		self.canvas.texture_creator()
	}

	pub fn set_fullscreen(&mut self, fullscreen: FullscreenType) {
		self.canvas.window_mut().set_fullscreen(fullscreen).unwrap();
	}
}