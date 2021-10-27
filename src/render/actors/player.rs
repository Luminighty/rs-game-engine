use sdl2::{pixels::Color, render::TextureCreator, video::WindowContext};
use crate::{game::{self, actor::{MouseState, Player}}, render::{Sprite, renderable::RenderSprite, renderable::{RenderRect, Renderable}}, utils::Vector2};
use super::{SdlWrapper, TextureMap};

pub fn render<'r>(
    sdl: &mut SdlWrapper,
    texture_creator: &'r TextureCreator<WindowContext>,
    textures: &mut TextureMap<'r>,
    game: &game::Game,
	app: &game::Application,
) {
	let frame = (app.frame / (game.animation_step / 2) % 4) as u8;

	let player = RenderSprite::from(&game.player)
										.sheet((frame as i32, 0));
	player.render(sdl, texture_creator, textures, app);

	let pos = &game.player.position.game_position();

	let outline = player_outline(*pos, &game.player.mouse_state);
	outline.map(|o| o.render(sdl, texture_creator, textures, app));

	//render_path(sdl, texture_creator, textures, game, app);
}

fn player_outline(position: Vector2, state: &MouseState) -> Option<RenderSprite> {
	match state {
		&MouseState::Hover => Some(RenderSprite::new(position, Sprite::Interact).sheet((1, 0))),
		&MouseState::Selected => None,
		_ => None,
	}
}

impl From<&Player> for RenderSprite {
	fn from(player: &Player) -> Self {
		RenderSprite::new(player.position.render_position(), Sprite::Player).offset(player.position.render_offset())
	}
}