use sdl2::{pixels::Color, render::TextureCreator, video::WindowContext};

use crate::{game, render::tile_rect, utils::{Vector2}};

use super::{Renderable, SdlWrapper, TextureMap};

pub struct RenderRect {
    color: Color,
    position: Vector2,
    _size: Option<Vector2>,
    _width: Option<u32>,
}

impl RenderRect {
    pub fn new(position: Vector2, color: Color) -> Self {
        Self {
            color,
            position,
            _size: None,
            _width: None,
        }
    }

    pub fn size(self, size: Vector2) -> Self {
        Self {
            color: self.color,
            position: self.position,
            _width: self._width,
            _size: Some(size),
        }
    }

    pub fn width(self, width: u32) -> Self {
        Self {
            color: self.color,
            position: self.position,
            _size: self._size,
            _width: Some(width),
        }
    }
}

impl Renderable for RenderRect {
    fn render<'r>(
        &self,
        sdl: &mut SdlWrapper,
        _texture_creator: &'r TextureCreator<WindowContext>,
        _textures: &mut TextureMap<'r>,
        app: &game::Application,
    ) {
        let Vector2 { x, y } = self.position;
        let size = self._size.unwrap_or((16, 16).into());

        let rect = tile_rect(x, y, size.x as u32, size.y as u32, app.upscale);
		let width = self._width.unwrap_or(app.upscale);

        sdl.canvas.set_draw_color(self.color);
        sdl.canvas.draw_rect(rect.into()).unwrap();
        for i in 1..width {
            sdl.canvas.draw_rect(rect.shrink(i as i32).into()).unwrap();
        }
    }
}
