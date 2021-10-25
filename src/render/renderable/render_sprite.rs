use sdl2::{render::TextureCreator, video::WindowContext};

use crate::{game, utils::Vector2};

use super::{tile_rect_offset, Renderable, SdlWrapper, Sprite, TextureMap};

pub struct RenderSprite {
    position: Vector2,
    offset: Vector2,
    sprite: Sprite,
    _sheet: Option<Vector2>,
}

impl RenderSprite {
    pub fn new<PVec2: Into<Vector2>, OVec2: Into<Vector2>>(position: PVec2, offset: OVec2, sprite: Sprite) -> Self {
        Self {
            position: position.into(),
            offset: offset.into(),
            sprite,
            _sheet: None,
        }
    }

    pub fn sheet<T: Into<Vector2>>(self, v: T) -> Self {
        Self {
            position: self.position,
            sprite: self.sprite,
            offset: self.offset,
            _sheet: Some(v.into()),
        }
    }
}

impl Renderable for RenderSprite {
    fn render<'r>(
        &self,
        sdl: &mut SdlWrapper,
        texture_creator: &'r TextureCreator<WindowContext>,
        textures: &mut TextureMap<'r>,
        app: &game::Application,
    ) {
        let position = self.position;
        let offset = self.offset;
        let sheet = self._sheet.unwrap_or((0, 0).into());
        let sprite = self.sprite;

        let (texture, rect) =
            textures.get_sheet(sprite, sheet.x as u8, sheet.y as u8, texture_creator);
        let dst = tile_rect_offset(
            position.x,
            position.y,
            offset.x,
            offset.y,
            rect.width,
            rect.height,
            app.upscale,
        );
        sdl.canvas
            .copy_ex(texture, rect, dst, 0.0, None, false, false)
            .unwrap();
    }
}
