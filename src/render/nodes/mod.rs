use std::convert::TryInto;

use sdl2::{render::TextureCreator, video::WindowContext, pixels::Color};

use crate::{game::{self}, log_err, utils::{Direction, DirectionMap, Vector2, pathfinder::PathNode}};
use super::{SdlWrapper, Sprite, TextureMap, renderable::{RenderRect, RenderSprite, Renderable}};

pub fn render<'r>(
    sdl: &mut SdlWrapper,
    texture_creator: &'r TextureCreator<WindowContext>,
    textures: &mut TextureMap<'r>,
    game: &game::Game,
	app: &game::Application,
) {

	if let Some(pathfinder) = &game.pathfinder {
		for i in 0..pathfinder.nodes.len() {
			let node = &pathfinder.nodes[i];

			if let Some(path) = &game.path {
				if path.iter().any(|&vec| vec == node.position().into()) {
					continue;
				}
			}

			/*
			let is_hovered = game.path.iter().any(|vec| vec.len() > 0 && vec[0] == node.position().into());
			if is_hovered {
				continue;
			}*/

			let position = Vector2::new(node.x, node.y);
			let rect = RenderSprite::new(position, Sprite::Interact);
			rect.render(sdl, texture_creator, textures, app);
	
		}
	}

	if let Some(path) = &game.path {
		let frame = (app.frame / game.animation_step) as i32;
		if path.len() != 0 {
			RenderSprite::new(path[0], Sprite::Interact).sheet((frame % 2, 1))
			.render(sdl, texture_creator, textures, app);

		}
		for i in 0..path.len() {
			let node = path[i];

			// TODO: Draw path
			let last = if i == 0 { None } else { path.get(i-1) };
			let next = path.get(i+1);

			let sprite = match (last, next) {
				(None, Some(next)) => draw_path_end(&node, next, frame),
				(Some(last), Some(next)) => draw_path_mid(&node, last, next, frame),
				(Some(last), None) => draw_path_start(&node, last, frame),
				(None, None) => continue,
			};

			if let Some(sprite) = sprite {
				sprite.render(sdl, texture_creator, textures, app);
			} else {
				crate::log_err!("Sprite not found for path! last: {:?} current: {:?} next: {:?}", last, node, next);
			}
		}
	}
}

fn draw_path_end(position: &Vector2, next: &Vector2, frame: i32) -> Option<RenderSprite> {
	let delta = *next - *position;
	let rotation = match delta.try_into() {
		Ok(Direction::Up) => Some(0.0), 
		Ok(Direction::Down) => Some(180.0), 
		Ok(Direction::Right) => Some(270.0), 
		Ok(Direction::Left) => Some(90.0), 
		_ => None
	};
	if let Some(r) = rotation {
		Some(RenderSprite::new(*position, Sprite::Interact).sheet((frame % 2, 2)).angle(r))
	} else {
		None
	}
}

fn draw_path_mid(position: &Vector2, last: &Vector2, next: &Vector2, frame: i32) -> Option<RenderSprite> {
	let mut map = DirectionMap::new();
	apply_map(&mut map, last, position);
	apply_map(&mut map, next, position);

	let data = match (map.is_horizontal(), map.is_vertical()) {
		(true, false) => Some((3, 90.0)),
		(false, true) => Some((3, 0.0)),
		(false, false) => match map {
			x if x.has_down() && x.has_right() => Some((4, 90.0)), // 
			x if x.has_up()   && x.has_right() => Some((4, 180.0)), // +
			x if x.has_down() && x.has_left()  => Some((4, 0.0)), // 
			x if x.has_up()   && x.has_left()  => Some((4, 270.0)), // +
			_ => None
		},
		_ => None,
	};

	if let Some((sprite, angle)) = data {
		Some(RenderSprite::new(*position, Sprite::Interact).sheet((0, sprite)).angle(angle))
	} else {
		None
	}
}

fn apply_map(map: &mut DirectionMap, from: &Vector2, to: &Vector2) {
	let delta = *to - *from;
	if let Ok(dir) = delta.try_into() {
		map.set(dir);
	}
}

fn draw_path_start(position: &Vector2, last: &Vector2, frame: i32) -> Option<RenderSprite> {
	let delta = *position - *last;
	let rotation = match delta.try_into() {
		Ok(Direction::Up) => Some(0.0), 
		Ok(Direction::Down) => Some(180.0), 
		Ok(Direction::Right) => Some(270.0), 
		Ok(Direction::Left) => Some(90.0), 
		_ => None
	};
	if let Some(r) = rotation {
		Some(RenderSprite::new(*position, Sprite::Interact).sheet((0, 5)).angle(r))
	} else {
		None
	}
}