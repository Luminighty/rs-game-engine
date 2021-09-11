use sdl2::{render::TextureCreator, video::WindowContext};
use crate::game;
use super::{SdlWrapper, TextureMap, tile_rect};

mod enemies;
mod player;

pub fn render_actors<'r>(
    sdl: &mut SdlWrapper,
    texture_creator: &'r TextureCreator<WindowContext>,
    textures: &mut TextureMap<'r>,
    game: &game::Game,
	app: &game::Application,
) {
	player::render_player(sdl, texture_creator, textures, game, app);
	enemies::render_enemies(sdl, texture_creator, textures, game, app);
}