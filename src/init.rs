extern crate sdl2;

use crate::game;
use crate::render;
use crate::config;
use crate::input;

pub fn init(config: &config::Config) -> (game::Application, game::Game, input::InputSystem, render::SdlWrapper) {
	let app = game::Application::new();
	let game = game::Game::new();
	let renderer = render::SdlWrapper::init(config);
	let input = input::InputSystem::init();

	(app, game, input, renderer)
}