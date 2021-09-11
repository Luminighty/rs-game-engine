extern crate sdl2;

mod sdl_wrapper;
mod sprite;
mod texture_map;

use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
pub use sdl_wrapper::SdlWrapper;
pub use sprite::{Sprite, SpriteData, Sprites};
pub use texture_map::TextureMap;

use crate::game;
use crate::input;
use crate::utils::Rect;
use sdl2::pixels::Color;

mod actors;
mod map;

pub fn render<'r>(
    sdl: &mut SdlWrapper,
    texture_creator: &'r TextureCreator<WindowContext>,
    textures: &mut TextureMap<'r>,
    game: &game::Game,
	app: &game::Application,
    _input: &input::InputSystem,
) {
    sdl.canvas.set_draw_color(Color::RGB(255, 255, 255));
    sdl.canvas.clear();

    map::render_map(sdl, texture_creator, textures, game, app);
	actors::render_actors(sdl, texture_creator, textures, game, app);

    sdl.canvas.present();
}


pub fn tile_rect(x: i32, y: i32, width: u32, height: u32, upscale: u32) -> Rect {
	Rect::new()
		.offset(x * width as i32, y * height as i32)
		.size(width, height)
		.scalar(upscale as i32)
}