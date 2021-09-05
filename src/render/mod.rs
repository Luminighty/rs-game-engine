extern crate sdl2;

mod texture_map;
mod sdl_wrapper;
mod sprite;

use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
pub use sdl_wrapper::SdlWrapper;
pub use sprite::{Sprite, SpriteData, Sprites};
pub use texture_map::TextureMap;

use crate::game;
use crate::input;
use sdl2::pixels::Color;

pub fn render<'r>(sdl: &mut SdlWrapper, texture_creator: &'r TextureCreator<WindowContext>, 
				textures: &mut TextureMap<'r>, 
				game: &game::Game, 
				input: &input::InputSystem) {

	sdl.canvas.set_draw_color(Color::RGB(0, 0, 0));
	sdl.canvas.clear();
	

	sdl.canvas.present();
}