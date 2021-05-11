use std::{env, marker::PhantomData};

use super::Module;
use crate::{Color, Powerline, Style};

pub struct ExitCode<S: ExitCodeScheme> {
	scheme: PhantomData<S>,
}

pub trait ExitCodeScheme {
	const EXIT_CODE_BG: Color;
	const EXIT_CODE_FG: Color;
}

impl<S: ExitCodeScheme> ExitCode<S> {
	pub fn new() -> ExitCode<S> {
		ExitCode { scheme: PhantomData }
	}
}

impl<S: ExitCodeScheme> Module for ExitCode<S> {
	fn append_segments(&mut self, powerline: &mut Powerline) {
		if let Some(exit_code) = env::args().nth(1).as_deref() {
			if exit_code != "0" {
				powerline.add_segment(exit_code, Style::simple(S::EXIT_CODE_FG, S::EXIT_CODE_BG))
			}
		}
	}
}
