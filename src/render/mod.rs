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
mod renderable;

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

pub const DEFAULT_UNIT: u32 = 16;

pub fn unit_tile_rect(x: i32, y: i32, upscale: u32) -> Rect {
    tile_rect(x, y, DEFAULT_UNIT, DEFAULT_UNIT, upscale)
}

pub fn tile_rect(x: i32, y: i32, width: u32, height: u32, upscale: u32) -> Rect {
    tile_rect_offset(x, y, 0, 0, width, height, upscale)
}

pub fn tile_rect_offset(x: i32, y: i32, offset_x: i32, offset_y: i32, width: u32, height: u32, upscale: u32) -> Rect {
	Rect::new()
		.offset(x * width as i32 + offset_x, y * height as i32 + offset_y)
		.size(width, height)
		.scalar(upscale as i32)
}