use sdl2::{event::{Event, WindowEvent}, video::FullscreenType};

use crate::{UNIT, game::{self, AppEvent}, input::{InputSystem}, utils::{self, Rect, Vector2}};

mod player;
mod debug;
mod nodes;

pub fn update(app: &mut game::Application, game: &mut game::Game, input: &InputSystem) {
	app.frame += 1;
	app.clear_events();

	player::update(app, game, input);
	nodes::update(app, game, input);
	debug::update(app, game, input);

	sdl_events(app, game, input);
}

pub fn get_actor_rect<Vec2: Into<utils::Vector2>>(vector: Vec2) -> utils::Rect {
	let vector: Vector2 = vector.into();
	utils::Rect::new().offset(vector.x * UNIT, vector.y * UNIT).size(UNIT as u32, UNIT as u32)
}

pub fn sdl_events(app: &mut game::Application, game: &mut game::Game, input: &InputSystem) {
	for event in input.events() {
		match event {
			Event::Window { win_event, .. } => match win_event {
				WindowEvent::SizeChanged(w, h) => resize(app, *w, *h),
				_ => (),
			}
			Event::KeyUp {keycode, ..} => if let Some(keycode) = keycode {
				key_up(app, *keycode);
			}
			_ => (),
		}
	}
}

pub fn key_up(app: &mut game::Application, keycode: sdl2::keyboard::Keycode) {
	use sdl2::keyboard::Keycode;
	match keycode {
		Keycode::F1 => app.push_event(AppEvent::SetFullscreen(FullscreenType::True)),
		Keycode::F2 => app.push_event(AppEvent::SetFullscreen(FullscreenType::Off)),
		_ => (),
	}
}

pub fn resize(app: &mut game::Application, width: i32, height: i32) {
	const TARGET: (i32, i32) = (14 * 16, 13 * 16);

	let upscale = (
		width / TARGET.0,
		height / TARGET.1,
	);
	let upscale = i32::max( i32::min(upscale.0, upscale.1), 1);

	app.push_event(AppEvent::SetUpscale(upscale as u32));


	let offset = (
		(width - TARGET.0 * upscale) / 2,
		(height - TARGET.1 * upscale) / 2,
	);
	let viewport = Rect::new()
		.offset(offset.0, offset.1)
		.size((TARGET.0 * upscale) as u32, (TARGET.1 * upscale) as u32);

	app.push_event(AppEvent::SetViewPort(Some(viewport)));
}