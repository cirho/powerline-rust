use super::Module;
use std::marker::PhantomData;

use crate::{Segment, R};
use crate::terminal::Color;
use std::path::Path;

pub struct PyVenv<S: PyVenvScheme> {
	scheme: PhantomData<S>,
}

pub trait PyVenvScheme {
	const PYVENV_FG: Color;
	const PYVENV_BG: Color;
	const PYVENV_SYMBOL: &'static str = "🐍";
}


impl<S: PyVenvScheme> PyVenv<S> {
	pub fn new() -> PyVenv<S> {
		PyVenv { scheme: PhantomData }
	}
}


impl<S: PyVenvScheme> Module for PyVenv<S> {
	fn append_segments(&mut self, segments: &mut Vec<Segment>) -> R<()> {
		let pyvenv = std::env::var("VIRTUAL_ENV")
			.or(std::env::var("CONDA_ENV_PATH"))
			.or(std::env::var("CONDA_DEFAULT_ENV"));
		match pyvenv {
			Ok(venv) => {
				Path::new(&venv)
					.file_name()
					.and_then(|venv_name| {
						segments.push(Segment::simple(
							format!(" {} {} ", S::PYVENV_SYMBOL, venv_name.to_string_lossy()),
							S::PYVENV_FG,
							S::PYVENV_BG,
						));
						Some(())
					});

				Ok(())
			}
			_ => { Ok(()) }
		}
	}
}