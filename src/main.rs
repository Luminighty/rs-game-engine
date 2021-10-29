// #![windows_subsystem = "windows"]

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

	const NANO_SEC: u128 = 1_000_000_000;

	log!("Initializing...");

	let config = config::default();
	let (mut app, mut game, mut input, mut sdlwrapper) = init::init(&config);
	
	sdl2::image::init(sdl2::image::InitFlag::PNG).unwrap();

	let texture_creator = sdlwrapper.texture_creator();
	let mut textures = render::TextureMap::init();

	log_success!("Game started");
	let mut time = std::time::Instant::now();
	loop {
		input::get_user_input(&mut input, &mut sdlwrapper, &mut app);
		update::update(&mut app, &mut game, &input);
		render::render(&mut sdlwrapper, &texture_creator, &mut textures,  &game, &app, &input);
		if !app.is_running {
			break;
		}
		let now = std::time::Instant::now();
		let duration = now.duration_since(time).as_nanos();
		// log!("Time since last update {}.{:03}ms", duration  / 1_000_000, duration % 1_000_000 / 1000);
		if duration < NANO_SEC {
			std::thread::sleep(std::time::Duration::new(0, (NANO_SEC - duration) as u32 / 60));
		}
		time = std::time::Instant::now();
	}
	
	log_success!("Game closed successfully...")
}