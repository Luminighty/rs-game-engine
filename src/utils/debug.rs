#[macro_export]
macro_rules! time {
	() => {
		{
			let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
			let millis = now.as_millis();
			let seconds = millis / 1000;
			let minutes = seconds / 60;
			let hours = minutes / 60;
			format!("{:02}:{:02}:{:02}:{:03}", hours % 24, minutes % 60, seconds % 60, millis % 1000)
		}
	}
}

#[macro_export]
macro_rules! cprintln {
	($color:expr, $kind:expr, $text:expr) => {
		println!("{}[{}] {}:{}:{} {}:\x1b[0m {}", $color, crate::time!(), file!(), line!(), column!(), $kind, $text)
	};
	($color:expr, $kind:expr, $format:expr, $($arg: expr),*) => {
		println!("{}[{}] {}:{}:{} {}:\x1b[0m {}", $color, crate::time!(), file!(), line!(), column!(), $kind, format!($format, $($arg),*))
	};
}

#[macro_export]
macro_rules! log {
	($( $arg: expr),*) => {
		crate::cprintln!("\x1b[1;37;40m", "    LOG", $($arg),*)
	};
}
#[macro_export]
macro_rules! log_warn {
	($( $arg: expr),*) => {
		crate::cprintln!("\x1b[1;33;40m", "   WARN", $($arg),*)
	};
}

#[macro_export]
macro_rules! log_err {
	($( $arg: expr),*) => {
		crate::cprintln!("\x1b[1;31;40m", "    ERR", $($arg),*)
	};
}
#[macro_export]
macro_rules! log_success {
	($( $arg: expr),*) => {
		crate::cprintln!("\x1b[1;32;40m", "SUCCESS", $($arg),*)
	};
}