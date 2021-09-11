use sdl2::{pixels::Color, render::TextureCreator, video::WindowContext};
use crate::{game::{self, actor::MouseState}, render::Sprite};
use super::{SdlWrapper, TextureMap, tile_rect};

pub fn render_player<'r>(
    sdl: &mut SdlWrapper,
    texture_creator: &'r TextureCreator<WindowContext>,
    textures: &mut TextureMap<'r>,
    game: &game::Game,
	app: &game::Application,
) {
	let position = &game.player.position;
	let frame = (app.frame / (game.animation_step / 2) % 4) as u8;
	let (texture, rect) = textures.get_sheet(Sprite::Player, frame, 0, texture_creator);
	let dst = tile_rect(position.x, position.y, rect.width, rect.height, app.upscale);

	if game.player.mouse_state == MouseState::Hover {
		sdl.canvas.set_draw_color(Color::RGB(50, 200, 100));
		sdl.canvas.draw_rect(dst.into()).unwrap();
		for i in 1..app.upscale {
			sdl.canvas.draw_rect(dst.shrink(i as i32).into()).unwrap();
		}
	}

	sdl.canvas.copy_ex(texture, rect, dst, 0.0, None, false, false).unwrap();
}