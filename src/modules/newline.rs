use std::{marker::PhantomData};
use super::Module;
use crate::{terminal::Color, Segment, R};
pub struct NewLine<S: NewLineScheme> {
	scheme: PhantomData<S>,
}

pub trait NewLineScheme {
	const NEWLINE_ROOT_BG: Color;
	const NEWLINE_BG: Color;
	const NEWLINE_FG: Color;
}

impl<S: NewLineScheme> NewLine<S> {
	pub fn new() -> NewLine<S> {
		NewLine {
			scheme: PhantomData,
		}
	}

	pub fn show_on_remote_shell() -> NewLine<S> {
		NewLine {
			scheme: PhantomData,
		}
	}
}

impl<S: NewLineScheme> Module for NewLine<S> {
	fn append_segments(&mut self, segments: &mut Vec<Segment>) -> R<()> {
		segments.push(Segment::simple(format!("\n"), S::NEWLINE_FG, S::NEWLINE_BG));
		Ok(())
	}
}
