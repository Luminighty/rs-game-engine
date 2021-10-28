extern crate sdl2;

mod sdl_wrapper;
mod sprite;
mod texture_map;

use sdl2::render::TextureCreator;
use sdl2::video::FullscreenType;
use sdl2::video::WindowContext;
pub use sdl_wrapper::SdlWrapper;
pub use sprite::{Sprite, SpriteData, Sprites};
pub use texture_map::TextureMap;

use crate::UNIT;
use crate::game;
use crate::game::AppEvent;
use crate::input;
use crate::utils::Rect;
use crate::utils::Vector2;
use sdl2::pixels::Color;

mod actors;
mod map;
mod renderable;
mod nodes;

pub fn render<'r>(
    sdl: &mut SdlWrapper,
    texture_creator: &'r TextureCreator<WindowContext>,
    textures: &mut TextureMap<'r>,
    game: &game::Game,
	app: &game::Application,
    _input: &input::InputSystem,
) {
    app_events(sdl, app);
    sdl.canvas.set_draw_color(sdl.meta.clear_color);
    sdl.canvas.clear();

    map::render(sdl, texture_creator, textures, game, app);
	actors::render(sdl, texture_creator, textures, game, app);
    nodes::render(sdl, texture_creator, textures, game, app);

    sdl.canvas.present();
}


pub fn unit_tile_rect<Vec2>(position: Vec2, upscale: u32) -> Rect 
where
    Vec2: Into<Vector2>
{
    tile_rect(position, (UNIT as i32, UNIT as i32), upscale)
}

pub fn tile_rect<Vec2I, Vec2U>(position: Vec2I, size: Vec2U, upscale: u32) -> Rect 
where 
    Vec2I: Into<Vector2>,
    Vec2U: Into<Vector2>,
{
    tile_rect_offset(position, (0, 0), size, upscale)
}

pub fn tile_rect_offset<Vec2IA, Vec2IB, Vec2U>(position: Vec2IA, offset: Vec2IB, size: Vec2U, upscale: u32) -> Rect
    where 
        Vec2IA: Into<Vector2>,
        Vec2IB: Into<Vector2>,
        Vec2U: Into<Vector2>,
    {
	let pos = position.into();
	let offset = offset.into();
	let size = size.into();
    Rect::new()
		.offset(pos.x * size.x + offset.x, pos.y * size.y + offset.y)
		.size(size.x as u32, size.y as u32)
		.scalar(upscale as i32)
}

pub fn app_events(sdl: &mut SdlWrapper, app: &game::Application) {
    for event in app.events() {
        match event {
            AppEvent::ResizeWindow {x, y} => {
                sdl.canvas.window_mut().set_size(*x, *y).unwrap();
                if sdl.canvas.window().fullscreen_state() != FullscreenType::True {
                    sdl.meta.windowed_size = (*x, *y);
                }
            }
            AppEvent::SetFullscreen(fullscreen_type) => {
                if *fullscreen_type == FullscreenType::True {
                    sdl.canvas.window_mut().maximize();
                }
                sdl.canvas.window_mut().set_fullscreen(*fullscreen_type).unwrap();
                if *fullscreen_type != FullscreenType::True {
                    sdl.canvas.window_mut().set_size(sdl.meta.windowed_size.0, sdl.meta.windowed_size.1).unwrap();
                }
            },
            AppEvent::SetUpscale(upscale) => sdl.meta.upscale = *upscale,
            AppEvent::SetViewPort(rect) => {
                let rect = rect.map(|r| r.into());
                sdl.canvas.set_viewport(rect);
                sdl.meta.viewport = rect;
            },
            // _ => (),
        }
    }
}