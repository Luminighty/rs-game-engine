use sdl2::{render::TextureCreator, video::WindowContext};

use crate::{game};

use super::{SdlWrapper, Sprite, TextureMap, tile_rect_offset};

mod render_rect;
mod render_sprite;

pub use render_rect::RenderRect;
pub use render_sprite::RenderSprite;

pub trait Renderable {
	fn render<'r>(&self, sdl: &mut SdlWrapper,
		texture_creator: &'r TextureCreator<WindowContext>,
		textures: &mut TextureMap<'r>,
		app: &game::Application);
}
