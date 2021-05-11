use std::{env, marker::PhantomData};

use super::Module;
use crate::{Color, Powerline, Style};

pub struct Cmd<S: CmdScheme> {
	status: Option<bool>,
	scheme: PhantomData<S>,
}

pub trait CmdScheme {
	const CMD_PASSED_FG: Color;
	const CMD_PASSED_BG: Color;
	const CMD_FAILED_BG: Color;
	const CMD_FAILED_FG: Color;
	const CMD_ROOT_SYMBOL: &'static str = "#";
	const CMD_USER_SYMBOL: &'static str = "$";
}

impl<S: CmdScheme> Cmd<S> {
	pub fn new() -> Cmd<S> {
		Cmd { status: None, scheme: PhantomData }
	}

	pub fn with_status(status: bool) -> Cmd<S> {
		Cmd { status: Some(status), scheme: PhantomData }
	}
}

impl<S: CmdScheme> Module for Cmd<S> {
	fn append_segments(&mut self, powerline: &mut Powerline) {
		let (fg, bg) = if self.status.or_else(|| env::args().nth(1).map(|x| x == "0")).unwrap_or(false) {
			(S::CMD_PASSED_FG, S::CMD_PASSED_BG)
		} else {
			(S::CMD_FAILED_FG, S::CMD_FAILED_BG)
		};

		let special = if users::get_current_uid() == 0 { S::CMD_ROOT_SYMBOL } else { S::CMD_USER_SYMBOL };
		powerline.add_segment(special, Style::simple(fg, bg));
	}
}
