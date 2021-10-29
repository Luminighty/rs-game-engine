

use sdl2::{render::TextureCreator, video::WindowContext};

use crate::{game::{self}};
use super::{SdlWrapper, TextureMap};

mod movement;
mod attack;

pub fn render<'r>(
    sdl: &mut SdlWrapper,
    texture_creator: &'r TextureCreator<WindowContext>,
    textures: &mut TextureMap<'r>,
    game: &game::Game,
	app: &game::Application,
) {
	movement::render(sdl, texture_creator, textures, game, app);
	attack::render(sdl, texture_creator, textures, game, app);
}
