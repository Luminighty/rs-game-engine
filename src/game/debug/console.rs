use std::str::Split;

use crate::game::{self};

pub type ConsoleResult = Result<Command, ConsoleErr>;

pub struct Console {
	pub enabled: bool,
	pub shown: bool,
	_command: String,
}

impl Console {
	pub fn new(enabled: bool) -> Self {
		Console {
			enabled,
			shown: false,
			_command: String::new(),
		}
	}

	pub fn parse(&mut self) -> ConsoleResult {
		let cmd = self._command.to_lowercase();
		let mut args = cmd.split(' ');
		let cmd = args.next();
		self._command = String::new();
		match cmd {
			Some("help") => Ok(Command::Help),
			Some("print") => print(args),
			Some("spawn") => spawn(&mut args),
			None | Some("") => Ok(Command::Empty),
			Some(unknown) => Err(ConsoleErr::CommandNotFound(String::from(unknown))),
		}
	}

	pub fn append(&mut self, chr: char) {
		self._command.push(chr)
	}

	pub fn command(&self) -> &String {
		&self._command
	}
	pub fn pop(&mut self) {
		self._command.pop();
	}
}

fn print(args: Split<char>) -> ConsoleResult {
	let msg = args.collect::<String>();
	Ok(Command::Print(msg))
}

fn spawn(args: &mut Split<char>) -> ConsoleResult {
	const USAGE: &str = "spawn object x y";

	let object = args.next().map(parse::<GameObject>);
	let x = args.next().map(parse::<i32>);
	let y = args.next().map(parse::<i32>);

	let object = object.unwrap_or(Err(ConsoleErr::MissingArg(USAGE.to_string())));
	let x = x.unwrap_or(Err(ConsoleErr::MissingArg(USAGE.to_string())));
	let y = y.unwrap_or(Err(ConsoleErr::MissingArg(USAGE.to_string())));

	match (object, x, y) {
		(Ok(object), Ok(x), Ok(y)) => Ok(Command::Spawn(object, x, y)),
		(object, x, y) => Err(object.and(x).and(y).unwrap_err()),
	}
}


#[derive(Debug, Clone)]
pub enum Command {
	Help,
	Print(String),
	Spawn(GameObject, i32, i32),
	Empty,
}

#[derive(Debug)]
pub enum ConsoleErr {
	ParseIntError(String),
	ParseObjectError(String),
	CommandNotFound(String),
	MissingArg(String),
}

fn parse<T: DebugParse>(arg: &str) -> Result<T, ConsoleErr> {
	DebugParse::parse(arg)
}

trait DebugParse {
	fn parse(arg: &str) -> Result<Self, ConsoleErr> where Self: Sized;
}

#[derive(Debug, Clone)]
pub enum GameObject {
	Player,
	Enemy(game::actor::enemy::Kind),
	Wall(u8),
	Ground(u8),
}

impl DebugParse for GameObject {
	fn parse(arg: &str) -> Result<Self, ConsoleErr>
	where Self: Sized {
		use game::actor::enemy::Kind;
		match arg {
			"player" => Ok(GameObject::Player),
			"bat" => Ok(GameObject::Enemy(Kind::Bat)),
			"ghost" => Ok(GameObject::Enemy(Kind::Ghost)),
			"wisp" => Ok(GameObject::Enemy(Kind::Wisp)),
			"zombie" => Ok(GameObject::Enemy(Kind::Zombie)),
			_ => Err(ConsoleErr::ParseObjectError(String::from(arg))),
		}
	}
}

impl DebugParse for i32 {
	fn parse(arg: &str) -> Result<Self, ConsoleErr>
	where Self: Sized {
		match arg.parse() {
			Ok(val) => Ok(val),
			Err(_) => Err(ConsoleErr::ParseIntError(String::from(arg)))
		}
	}
}
