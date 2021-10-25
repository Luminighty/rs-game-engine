use sdl2::{render::TextureCreator, video::WindowContext};
use crate::{game::{self, map::Tile}, render::Sprite};
use super::{SdlWrapper, TextureMap, renderable::{RenderSprite, Renderable}};

pub fn render_map<'r>(
    sdl: &mut SdlWrapper,
    texture_creator: &'r TextureCreator<WindowContext>,
    textures: &mut TextureMap<'r>,
    game: &game::Game,
	app: &game::Application,
) {
	
    let map = &game.map;
	let width = &map.get_size().x;
    for index in 0..map.len() {
        if let Some((x, y)) = get_sheet(map.get_index(index)) {
            let position = (index as i32 % width, index as i32 / width);
            let sprite = RenderSprite::new(position, (0, 0), Sprite::Ground).sheet((x as i32, y as i32));

            sprite.render(sdl, texture_creator, textures, app);
        }
    }

	for (position, wall) in &map.walls {
		let frame = (app.frame / game.animation_step % 2) as i32;
        let position = *position;
        let sprite = RenderSprite::new(position, (0, 0), Sprite::Wall).sheet((*wall as i32, frame));
        sprite.render(sdl, texture_creator, textures, app);
	}
}

fn get_sheet(tile: Option<&Tile>) -> Option<(u8, u8)> {
    match tile {
        Some(Tile::Void) => Some((1, 2)),
        Some(Tile::Ground) => Some((0, 2)),
        Some(Tile::Flower(v)) => Some((5, *v)),
        Some(Tile::Grass(v)) => Some((2 + v % 2, v / 2)),
        Some(Tile::Tile(v)) => Some((2 + v, 2)),
        Some(Tile::Stick(v)) => Some((4, *v)),
        None => None,
    }
}