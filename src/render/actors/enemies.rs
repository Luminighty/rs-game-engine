use sdl2::{render::TextureCreator, video::WindowContext};
use crate::{game, render::Sprite};
use super::{SdlWrapper, TextureMap, tile_rect};

pub fn render_enemies<'r>(
    sdl: &mut SdlWrapper,
    texture_creator: &'r TextureCreator<WindowContext>,
    textures: &mut TextureMap<'r>,
    game: &game::Game,
	app: &game::Application,
) {
	for enemy in &game.enemies {

		let position = &enemy.position;
		let frame = (app.frame / game.animation_step % 2) as u8;
		let (texture, rect) = textures.get_sheet(Sprite::Undead, frame, enemy.kind.offset(), texture_creator);
		let dst = tile_rect(position.x, position.y, rect.width, rect.height, app.upscale);

		sdl.canvas.copy_ex(texture, rect, dst, 0.0, None, false, false).unwrap();

	}

}