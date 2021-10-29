
use sdl2::{render::TextureCreator, video::WindowContext};
use crate::game;
use crate::game::nodes::attack::AttackNode;
use crate::render::renderable::{RenderSprite, Renderable};
use crate::render::{SdlWrapper, Sprite, TextureMap, animation_frame};


pub fn render<'r>(
	sdl: &mut SdlWrapper,
	texture_creator: &'r TextureCreator<WindowContext>,
	textures: &mut TextureMap<'r>,
	game: &game::Game,
	app: &game::Application,
) {

	if let Some(attacks) = &game.nodes.attack {
		for ((x, y), node) in attacks {
			if let Some(hover) = &game.nodes.attack_hover {
				if hover == &(*x, *y) {
					hover_enemy(*x, *y, app.frame, game.framerate).render(sdl, texture_creator, textures, app);
					continue;
				}
			}
			let rect = match node {
				&AttackNode::Enemy(_) => enemy_node(*x, *y, app.frame, game.framerate),
				&AttackNode::Empty => empty_node(*x, *y),
			};
			rect.render(sdl, texture_creator, textures, app);
		}
	}
}

fn hover_enemy(x: i32, y: i32, frame: usize, framerate: usize) -> RenderSprite {
	let frame = animation_frame(frame, framerate, 1);
	RenderSprite::new((x, y), Sprite::Interact).sheet((2 + frame % 2, 5))
}

fn empty_node(x: i32, y: i32) -> RenderSprite {
	RenderSprite::new((x, y), Sprite::Interact).sheet((3, 0))
}

fn enemy_node(x: i32, y: i32, frame: usize, framerate: usize) -> RenderSprite {
	let frame = animation_frame(frame, framerate, 2);
	RenderSprite::new((x, y), Sprite::Interact).sheet((2, frame * 2 % 5))
}