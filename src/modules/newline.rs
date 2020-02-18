pub struct NewLine<S: NewLineScheme> {
	show_on_local: bool,
	scheme: PhantomData<S>,
}

pub trait NewLineScheme {
	const USERNAME_ROOT_BG: Color;
	const USERNAME_BG: Color;
	const USERNAME_FG: Color;
}

impl<S: NewLineScheme> NewLine<S> {
	pub fn new() -> NewLine<S> {
		NewLine {
			show_on_local: true,
			scheme: PhantomData,
		}
	}

	pub fn show_on_remote_shell() -> NewLine<S> {
		NewLine {
			show_on_local: false,
			scheme: PhantomData,
		}
	}
}

impl<S: NewLineScheme> Module for NewLine<S> {
	fn append_segments(&mut self, segments: &mut Vec<Segment>) -> R<()> {
		if self.show_on_local || utils::is_remote_shell() {
			let bg = if user.uid() == 0 { S::USERNAME_ROOT_BG } else { S::USERNAME_BG };

			segments.push(Segment::simple(format!("\n", S::USERNAME_FG, bg));
		}
		Ok(())
	}
}
