use sdl2::{render::TextureCreator, video::WindowContext};

use crate::{game, render::{Sprite}, utils::{Rect, Vector2}};

use super::{Renderable, SdlWrapper, TextureMap};

pub struct RenderText {
	text: String,
	font: Sprite,
	position: Vector2,
	_color: (u8, u8, u8),
}

#[allow(dead_code)]
impl RenderText {
    pub fn new<Vec2: Into<Vector2>>(position: Vec2, text: String, font: Sprite) -> Self {
        Self {
			position: position.into(),
			text, font,
			_color: (0,0,0),
		}
    }

	pub fn color(mut self, r: u8, g: u8, b: u8) -> Self {
		self._color = (r,g,b);
		self
	}
}

const ALPHABET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ _+->";

impl Renderable for RenderText {
    fn render<'r>(
        &self,
        sdl: &mut SdlWrapper,
        texture_creator: &'r TextureCreator<WindowContext>,
        textures: &mut TextureMap<'r>,
        _app: &game::Application,
    ) {
		let position = self.position * sdl.meta.upscale as i32;
		let text = self.text.to_uppercase();
		let sprite = self.font;
		let (red, green, blue) = self._color;

		let mut i = 0;
		for c in text.chars() {
			match ALPHABET.find(|chr| chr == c) {
				None => crate::log_err!("Undefined character: {}", c),
				Some(index) => {
					let (texture, src) = textures.get_sheet(sprite, index as u8, 0, texture_creator);
					texture.set_color_mod(red, green, blue);
					let dst = Rect::new().offset(position.x + i * 4 * sdl.meta.upscale as i32, position.y).size(3 * sdl.meta.upscale, 5* sdl.meta.upscale);
					sdl.canvas
					.copy_ex(
						texture,
						src,
						dst,
						0.0,
						None,
						false,
						false,
					)
					.unwrap();
				}
			}
			
			i += 1;
		}
    }
}
