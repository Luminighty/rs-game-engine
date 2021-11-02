use crate::game::{self, actor::Enemy, debug::console::{Command, ConsoleErr, GameObject}};


pub fn console(keycode: sdl2::keyboard::Keycode, game: &mut game::Game) {
	use sdl2::keyboard::Keycode;
	if game.console.shown {
		match keycode {
			Keycode::Return => {
				game.console.command().chars().for_each(|c| crate::log!("cmd char: {}", c as i32));
				match game.console.parse() {
					Ok(cmd) => execute_cmd(game, cmd),
					Err(err) => print_err(game, err),
				}
			},
			Keycode::Backspace => {
				game.console.pop();
			},
			chr => if let Some(c) = char::from_u32(chr as u32) {
				game.console.append(c);
				crate::log!("CMD: {}", game.console.command());

			}
		}
	}
}

fn print_err(game: &mut game::Game, err: ConsoleErr) {
	crate::log_err!("CMD error: {:?}", err);
}

fn execute_cmd(game: &mut game::Game, cmd: Command) {
	crate::log_success!("Command: {:?}", cmd);
	match cmd {
		Command::Empty => {},
		Command::Print(text) => crate::log!(text),
		Command::Help => help(),
		Command::Spawn(object, x, y) => spawn(game, object, x, y),
	}
}

fn spawn(game: &mut game::Game, object: GameObject, x: i32, y: i32) {
	use game::actor::enemy::Kind;
	match object {
		GameObject::Player => {},
		GameObject::Enemy(Kind::Bat)    => game.enemies.push(Enemy::bat((x, y).into())),
		GameObject::Enemy(Kind::Ghost)  => game.enemies.push(Enemy::ghost((x, y).into())),
		GameObject::Enemy(Kind::Zombie) => game.enemies.push(Enemy::zombie((x, y).into())),
		GameObject::Enemy(Kind::Wisp)   => game.enemies.push(Enemy::wisp((x, y).into())),
		GameObject::Wall(_) => {},
		GameObject::Ground(_) => {},
	}
}

fn help() {
	crate::log!("HELP")
}