use sdl2::video::FullscreenType;

use crate::utils::Rect;

#[derive(Debug, Clone)]
pub enum AppEvent {
	SetFullscreen(FullscreenType),
	ResizeWindow {x: u32, y: u32},
	SetViewPort(Option<Rect>),
	SetUpscale(u32),
	
}