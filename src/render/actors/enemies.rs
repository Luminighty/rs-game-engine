use sdl2::{render::TextureCreator, video::WindowContext};
use crate::{game::{self, actor::Enemy}, render::{Sprite, renderable::{RenderSprite, Renderable}}};
use super::{SdlWrapper, TextureMap};

pub fn render_enemies<'r>(
    sdl: &mut SdlWrapper,
    texture_creator: &'r TextureCreator<WindowContext>,
    textures: &mut TextureMap<'r>,
    game: &game::Game,
	app: &game::Application,
) {
	for enemy in &game.enemies {
		let enemy = RenderSprite::from(enemy);
		enemy.render(sdl, texture_creator, textures, app);
	}
}

impl From<&Enemy> for RenderSprite {
	fn from(enemy: &Enemy) -> Self {
		RenderSprite::new(enemy.position, enemy.offset, Sprite::Player)
	}
}