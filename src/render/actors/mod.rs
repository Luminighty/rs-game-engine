use sdl2::{render::TextureCreator, video::WindowContext};
use crate::game;
use super::{SdlWrapper, TextureMap};

mod enemies;
mod player;

pub fn render<'r>(
    sdl: &mut SdlWrapper,
    texture_creator: &'r TextureCreator<WindowContext>,
    textures: &mut TextureMap<'r>,
    game: &game::Game,
	app: &game::Application,
) {
	player::render(sdl, texture_creator, textures, game, app);
	enemies::render(sdl, texture_creator, textures, game, app);
}