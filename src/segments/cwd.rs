use color::Color;
use part::*;
use powerline::*;
use std::{env, path};

pub struct Cwd {
	special: &'static str,
	max_length: usize,
	wanted_seg_num: usize,
	resolve_symlinks: bool,
}

impl Cwd {
	pub fn new(special: &'static str, max_length: usize, wanted_seg_num: usize, resolve_symlinks: bool) -> Cwd {
		Cwd {
			special,
			max_length,
			wanted_seg_num,
			resolve_symlinks,
		}
	}
}

fn append_cwd_segments<'a, I>(segments: &mut Vec<Segment>, iter: I)
where
	I: Iterator<Item = &'a str>,
{
	for val in iter {
		segments.push(Segment::special(&format!(" {} ", val), Color::PATH_FG, Color::PATH_BG, '\u{E0B1}', Color::SEPARATOR_FG));
	}
}

impl Part for Cwd {
	fn get_segments(self) -> Result<Vec<Segment>, Error> {
		let current_dir = if self.resolve_symlinks {
			env::current_dir()?
		} else {
			path::PathBuf::from(env::var("PWD")?)
		};

		let mut cwd = current_dir.to_str().unwrap();
		let mut segments = Vec::new();

		if let Some(home_path) = env::home_dir() {
			let home_str = home_path.to_str().unwrap();
			if cwd.starts_with(home_str) {
				segments.push(Segment::simple(&format!(" {} ", self.special), Color::HOME_FG, Color::HOME_BG));
				cwd = &cwd[home_str.len()..]
			}
		}

		let depth = cwd.matches("/").count() - 1;
		if (cwd.len() > self.max_length as usize) && (depth > self.wanted_seg_num) {
			let left = self.wanted_seg_num / 2;
			let right = self.wanted_seg_num - left;

			let start = cwd.split("/").skip(1).take(left);
			let end = cwd.split("/").skip(depth - right + 2);

			append_cwd_segments(&mut segments, start);
			segments.push(Segment::special(" \u{2026} ", Color::PATH_FG, Color::PATH_BG, '\u{E0B1}', Color::SEPARATOR_FG));
			append_cwd_segments(&mut segments, end);
		} else {
			append_cwd_segments(&mut segments, cwd.split("/").skip(1));
		};

		if let Some(last) = segments.last_mut() {
			if last.val == "  " {
				last.val = " / ".to_string()
			}
			last.fg = Color::CWD_FG;
			last.sep = '\u{E0B0}';
			last.sep_col = last.bg;
		}

		Ok(segments)
	}
}
