use sdl2::{render::TextureCreator, video::WindowContext};
use crate::{game, render::Sprite};
use super::{SdlWrapper, TextureMap, tile_rect};

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