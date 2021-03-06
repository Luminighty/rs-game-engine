extern crate sdl2;


use sdl2::image::LoadTexture;

use crate::render;

pub struct TextureMap<'r> {
	map: std::collections::HashMap<String, sdl2::render::Texture<'r>>,
	sprites: render::Sprites,
}

impl<'r> TextureMap<'r> {
	pub fn init() -> Self {
		Self {
			map: std::collections::HashMap::new(),
			sprites: render::Sprites::init(),
		}
	}

	pub fn get(&mut self, sprite: render::Sprite, texture_creator: &'r sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> &sdl2::render::Texture<'r> {
		let fields = self.sprites.fields();
		let data = &fields[sprite as usize];
		if !self.map.contains_key(&data.path) {
			self.map.insert(String::from(&data.path), texture_creator.load_texture(&data.path).unwrap());
		}
		self.map.get(&data.path).unwrap()
	}
}