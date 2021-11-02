use sdl2::{render::TextureCreator, video::WindowContext};
use crate::{game::{self}, render::Sprite};
use super::{SdlWrapper, TextureMap, renderable::{RenderText, Renderable}};

pub fn render<'r>(
    sdl: &mut SdlWrapper,
    texture_creator: &'r TextureCreator<WindowContext>,
    textures: &mut TextureMap<'r>,
    game: &game::Game,
	app: &game::Application,
) {

	if game.console.shown {
		console(sdl, texture_creator, textures, game, app);
	}
}


fn console<'r>(
    sdl: &mut SdlWrapper,
    texture_creator: &'r TextureCreator<WindowContext>,
    textures: &mut TextureMap<'r>,
    game: &game::Game,
	app: &game::Application,
) {
    let cmd = String::from("> ") + game.console.command();
	RenderText::new((0, 0), cmd, Sprite::Font)
        .color(0, 255, 0)
        .render(sdl, texture_creator, textures, app);
}