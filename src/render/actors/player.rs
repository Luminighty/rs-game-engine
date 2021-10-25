use sdl2::{pixels::Color, render::TextureCreator, video::WindowContext};
use crate::{game::{self, actor::{MouseState, Player}}, render::{Sprite, renderable::RenderSprite, renderable::{RenderRect, Renderable}}};
use super::{SdlWrapper, TextureMap, tile_rect};

pub fn render_player<'r>(
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

	let pos = &game.player.position;
	let outline = outline_color(&game.player.mouse_state);
	let outline = outline.map(|c| RenderRect::new(*pos, c));
	outline.map(|o| o.render(sdl, texture_creator, textures, app));

	//render_path(sdl, texture_creator, textures, game, app);
}

fn outline_color(state: &MouseState) -> Option<Color> {
	match state {
		&MouseState::Hover => Some(Color::RGB(50, 200, 100)),
		&MouseState::Selected => Some(Color::RGB(80, 100, 150)),
		_ => None,
	}
}

fn render_path<'r>(
    sdl: &mut SdlWrapper,
    texture_creator: &'r TextureCreator<WindowContext>,
    textures: &mut TextureMap<'r>,
    game: &game::Game,
	app: &game::Application,
) {
	// Draw pathfinder tiles
	if let Some(result) = &game.player.path {
		sdl.canvas.set_draw_color(Color::RGB(50, 180, 70));
		for (x, y) in result.nodes().iter() {
			let dst = tile_rect(*x, *y, 16, 16, app.upscale);
			sdl.canvas.draw_rect(dst.into()).unwrap();
		}
	}
}

impl From<&Player> for RenderSprite {
	fn from(player: &Player) -> Self {
		RenderSprite::new(player.position, player.offset, Sprite::Player)
	}
}