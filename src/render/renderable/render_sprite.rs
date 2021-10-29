use sdl2::{rect::Point, render::TextureCreator, video::WindowContext};

use crate::{game, utils::Vector2};

use super::{tile_rect_offset, Renderable, SdlWrapper, Sprite, TextureMap};

pub struct RenderSprite {
    position: Vector2,
    sprite: Sprite,
    _sheet: Vector2,
    _offset: Vector2,
    _angle: f64,
    _flip: [bool; 2],
    _center: Option<Vector2>,
}

#[allow(dead_code)]
impl RenderSprite {
    pub fn new<Vec2: Into<Vector2>>(position: Vec2, sprite: Sprite) -> Self {
        Self {
            position: position.into(),
            sprite,
            _offset: Vector2::zero(),
            _sheet: Vector2::zero(),
            _angle: 0.0,
            _flip: [false, false],
            _center: None,
        }
    }

    pub fn sheet<Vec2: Into<Vector2>>(mut self, v: Vec2) -> Self {
        self._sheet = v.into();
        self
    }

    pub fn offset<Vec2: Into<Vector2>>(mut self, v: Vec2) -> Self {
        self._offset = v.into();
        self
    }

    pub fn flip(mut self, horizontal: bool, vertical: bool) -> Self {
        self._flip = [horizontal, vertical];
        self
    }

    pub fn center<Vec2: Into<Vector2>>(mut self, v: Vec2) -> Self {
        self._center = Some(v.into());
        self
    }

    pub fn angle(mut self, angle: f64) -> Self {
        self._angle = angle;
        self
    }
}

impl Renderable for RenderSprite {
    fn render<'r>(
        &self,
        sdl: &mut SdlWrapper,
        texture_creator: &'r TextureCreator<WindowContext>,
        textures: &mut TextureMap<'r>,
        _app: &game::Application,
    ) {
        let position = self.position;
        let offset = self._offset;
        let sheet = self._sheet;
        let sprite = self.sprite;
        let angle = self._angle;
        let [flip_horizontal, flip_vertical] = self._flip;
        let center = self._center.map(|c| Point::new(c.x, c.y));

        let (texture, src) =
            textures.get_sheet(sprite, sheet.x as u8, sheet.y as u8, texture_creator);
        let dst = tile_rect_offset(position, offset, (src.width, src.height), sdl.meta.upscale);
        sdl.canvas
            .copy_ex(
                texture,
                src,
                dst,
                angle,
                center,
                flip_horizontal,
                flip_vertical,
            )
            .unwrap();
    }
}
