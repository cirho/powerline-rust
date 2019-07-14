use std::env;

use crate::{color::Color, part::*, powerline::*, Error};

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
		let (fg, bg) = if self.status.or_else(|| env::args().nth(1).map(|x| x == "0")).unwrap_or(false) {
			(Color::CMD_PASSED_FG, Color::CMD_PASSED_BG)
		} else {
			(Color::CMD_FAILED_FG, Color::CMD_FAILED_BG)
		};
		let is_root = users::get_current_uid() == 0;
		let special = if is_root { self.root } else { self.normal };
		Ok(vec![Segment::simple(&format!(" {} ", special), fg, bg)])
	}
}
