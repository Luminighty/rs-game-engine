
use sdl2::{render::TextureCreator, video::WindowContext};
use crate::game;
use crate::game::nodes::attack::AttackNode;
use crate::render::renderable::{RenderSprite, Renderable};
use crate::render::{SdlWrapper, Sprite, TextureMap, animation_frame};
use crate::utils::Vector2;


pub fn render<'r>(
	sdl: &mut SdlWrapper,
	texture_creator: &'r TextureCreator<WindowContext>,
	textures: &mut TextureMap<'r>,
	game: &game::Game,
	app: &game::Application,
) {

	if let Some(attacks) = &game.nodes.attack {
		for (pos, node) in attacks {
			if let Some(hover) = &game.nodes.attack_hover {
				if hover == pos {
					hover_enemy(pos, app.frame, game.framerate).render(sdl, texture_creator, textures, app);
					continue;
				}
			}
			let rect = match node {
				&AttackNode::Enemy(_) => enemy_node(pos, app.frame, game.framerate),
				&AttackNode::Empty => empty_node(pos),
				&AttackNode::Origin => origin_node(pos),
			};
			rect.render(sdl, texture_creator, textures, app);
		}
	}
}

fn hover_enemy(pos: &Vector2, frame: usize, framerate: usize) -> RenderSprite {
	let frame = animation_frame(frame, framerate, 1);
	RenderSprite::new(pos, Sprite::Interact).sheet((2 + frame % 2, 5))
}

fn empty_node(pos: &Vector2) -> RenderSprite {
	RenderSprite::new(pos, Sprite::Interact).sheet((3, 0))
}

fn enemy_node(pos: &Vector2, frame: usize, framerate: usize) -> RenderSprite {
	let frame = animation_frame(frame, framerate, 2);
	RenderSprite::new(pos, Sprite::Interact).sheet((2, frame * 2 % 5))
}

fn origin_node(pos: &Vector2) -> RenderSprite {
	RenderSprite::new(pos, Sprite::Interact).sheet((3, 1))
}