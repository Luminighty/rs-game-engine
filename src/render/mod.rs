extern crate sdl2;
use sdl2::pixels::Color;

mod sdl_wrapper;

pub use sdl_wrapper::SdlWrapper;

use crate::game;


pub fn render(sdl: &mut SdlWrapper, game: &game::Game) {
	let i = (game.frame % 255) as u8;

	sdl.canvas.set_draw_color(Color::RGB(i, 64, 255-i));
	sdl.canvas.clear();

	

	sdl.canvas.present();
}