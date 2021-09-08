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

pub fn render<'r>(
    sdl: &mut SdlWrapper,
    texture_creator: &'r TextureCreator<WindowContext>,
    textures: &mut TextureMap<'r>,
    game: &game::Game,
	app: &game::Application,
    _input: &input::InputSystem,
) {
    sdl.canvas.set_draw_color(Color::RGB(0, 0, 0));
    sdl.canvas.clear();

    render_map(sdl, texture_creator, textures, game, app);
	render_actors(sdl, texture_creator, textures, game, app);

    sdl.canvas.present();
}

pub fn render_map<'r>(
    sdl: &mut SdlWrapper,
    texture_creator: &'r TextureCreator<WindowContext>,
    textures: &mut TextureMap<'r>,
    game: &game::Game,
	app: &game::Application,
) {
    use game::map::Tile;
	
    let map = &game.map;
	let width = &map.get_size().x;
    for index in 0..map.len() {
        let tile = match map.get_index(index) {
            Some(Tile::Void) => Some((1, 2)),
            Some(Tile::Ground) => Some((0, 2)),
            Some(Tile::Flower(v)) => Some((5, *v)),
            Some(Tile::Grass(v)) => Some((2 + v % 2, v / 2)),
            Some(Tile::Tile(v)) => Some((2 + v, 2)),
            Some(Tile::Stick(v)) => Some((4, *v)),
            None => None,
        };
		if let Some((x, y)) = tile {
			let (texture, rect) = textures.get_sheet(Sprite::Ground, x, y, texture_creator);
			let dst = tile_rect(index as i32 % width, index as i32 / width, rect.width, rect.height, app.upscale);
			sdl.canvas.copy_ex(texture, rect, dst, 0.0, None, false, false).unwrap();
		}
    }

	for (position, wall) in &map.walls {
		let frame = (app.frame / game.animation_step % 2) as u8;
		let (texture, rect) = textures.get_sheet(Sprite::Wall, *wall, frame, texture_creator);
		let dst = tile_rect(position.x, position.y, rect.width, rect.height, app.upscale);
		sdl.canvas.copy_ex(texture, rect, dst, 0.0, None, false, false).unwrap();
	}
}


pub fn render_actors<'r>(
    sdl: &mut SdlWrapper,
    texture_creator: &'r TextureCreator<WindowContext>,
    textures: &mut TextureMap<'r>,
    game: &game::Game,
	app: &game::Application,
) {
	{
		let position = &game.player.position;
		let frame = (app.frame / (game.animation_step / 2) % 4) as u8;
		let (texture, rect) = textures.get_sheet(Sprite::Player, frame, 0, texture_creator);
		let dst = tile_rect(position.x, position.y, rect.width, rect.height, app.upscale);

		sdl.canvas.copy_ex(texture, rect, dst, 0.0, None, false, false).unwrap();
	}

	for enemy in &game.enemies {

		let position = &enemy.position;
		let frame = (app.frame / game.animation_step % 2) as u8;
		let (texture, rect) = textures.get_sheet(Sprite::Undead, frame, enemy.kind.offset(), texture_creator);
		let dst = tile_rect(position.x, position.y, rect.width, rect.height, app.upscale);

		sdl.canvas.copy_ex(texture, rect, dst, 0.0, None, false, false).unwrap();

	}

}

fn tile_rect(x: i32, y: i32, width: u32, height: u32, upscale: u32) -> Rect {
	Rect::new()
		.offset(x * width as i32, y * height as i32)
		.size(width, height)
		.scalar(upscale as i32)
}