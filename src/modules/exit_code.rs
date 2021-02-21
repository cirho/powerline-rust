use std::{env, marker::PhantomData};

use super::Module;
use crate::{powerline::Segment, terminal::Color, R};

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
	fn append_segments(&mut self, segments: &mut Vec<Segment>) -> R<()> {
		let exit_code = env::args().nth(1).unwrap_or_else(|| "1".to_string());

		if exit_code != "0" {
			let (fg, bg) = (S::EXIT_CODE_FG, S::EXIT_CODE_BG);
			segments.push(Segment::simple(format!(" {} ", exit_code), fg, bg));
		}

		Ok(())
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::theme::SimpleTheme;

	#[test]
	fn exit_code_returns_single_segment() {
		let mut exit_code = ExitCode::<SimpleTheme>::new();
		let segments = exit_code.get_segments().unwrap();

		assert_eq!(1, segments.len())
	}
}
