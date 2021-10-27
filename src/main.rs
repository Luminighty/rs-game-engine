extern crate sdl2;

pub const UNIT: i32 = 16;

pub mod config;
pub mod game;
pub mod render;
pub mod init;
pub mod update;
pub mod input;
pub mod utils;

fn main() {

	log!("Initializing...");

	let config = config::default();
	let (mut app, mut game, mut input, mut sdlwrapper) = init::init(&config);
	
	sdl2::image::init(sdl2::image::InitFlag::PNG).unwrap();

	let texture_creator = sdlwrapper.texture_creator();
	let mut textures = render::TextureMap::init();

	log!("Help: {}:{}", file!(), line!());

	log_success!("Game started");
	loop {
		input::get_user_input(&mut input, &mut sdlwrapper, &mut app);
		update::update(&mut app, &mut game, &input);
		render::render(&mut sdlwrapper, &texture_creator, &mut textures,  &game, &app, &input);
		if !app.is_running {
			break;
		}
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
	}
	
	log_success!("Game closed successfully...")
}