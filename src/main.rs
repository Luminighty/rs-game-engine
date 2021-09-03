extern crate sdl2;

pub mod config;
pub mod game;
pub mod render;
pub mod init;
pub mod update;
pub mod input;


fn main() {
	let config = config::default();
	let (mut app, mut game, mut input, mut sdlwrapper) = init::init(&config);
	loop {
		input::get_user_input(&mut input, &mut sdlwrapper, &mut app);
		update::update(&mut app, &mut game, &input);
		render::render(&mut sdlwrapper, &game);
		if !app.is_running {
			break;
		}
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
	}
	println!("Game closed successfully...")
}