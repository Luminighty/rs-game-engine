use sdl2::{render::TextureCreator, video::WindowContext};
use crate::{game::{self, actor::Enemy}, render::{Sprite, animation_frame, renderable::{RenderSprite, Renderable}}};
use super::{SdlWrapper, TextureMap};

pub fn render<'r>(
    sdl: &mut SdlWrapper,
    texture_creator: &'r TextureCreator<WindowContext>,
    textures: &mut TextureMap<'r>,
    game: &game::Game,
	app: &game::Application,
) {
	let frame = animation_frame(app.frame, game.framerate, 1);
	for enemy in &game.enemies {
		let enemy = RenderSprite::from(enemy).sheet((frame % 2, enemy.kind.offset() as i32));
		enemy.render(sdl, texture_creator, textures, app);
	}
}

impl From<&Enemy> for RenderSprite {
	fn from(enemy: &Enemy) -> Self {
		RenderSprite::new(enemy.position, Sprite::Undead).offset(enemy.offset)
	}
}