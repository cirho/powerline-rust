use color::Color;
use part::*;
use powerline::*;
use std::env;
use users;

pub struct Cmd {
	normal: &'static str,
	root: &'static str,
	status: Option<bool>,
}

impl Cmd {
	pub fn new(normal: &'static str, root: &'static str) -> Cmd {
		Cmd { normal, root, status: None }
	}

	pub fn with_status(normal: &'static str, root: &'static str, status: bool) -> Cmd {
		Cmd {
			normal,
			root,
			status: Some(status),
		}
	}
}

impl Part for Cmd {
	fn get_segments(self) -> Result<Vec<Segment>, Error> {
		let (fg, bg) = if self.status.unwrap_or_else(|| env::args().nth(1).unwrap_or("".to_owned()) == "0") {
			(Color::CMD_PASSED_FG, Color::CMD_PASSED_BG)
		} else {
			(Color::CMD_FAILED_FG, Color::CMD_FAILED_BG)
		};
		let is_root = users::get_current_uid() == 0;
		let special = if is_root { self.root } else { self.normal };
		Ok(vec![Segment::simple(&format!(" {} ", special), fg, bg)])
	}
}
