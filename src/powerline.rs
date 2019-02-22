use color::Color;
use std::fmt;
use theme::Theme;

pub struct Segment {
	pub val: String,
	pub fg: Color,
	pub bg: Color,
	pub sep: char,
	pub sep_col: Color,
}

impl Segment {
	pub fn simple(val: &str, fg: Color, bg: Color) -> Segment {
		Segment {
			val: val.to_string(),
			fg,
			bg: bg.clone(),
			sep: '\u{E0B0}',
			sep_col: bg,
		}
	}

	pub fn special(val: &str, fg: Color, bg: Color, sep: char, sep_col: Color) -> Segment {
		Segment {
			val: val.to_string(),
			fg,
			bg,
			sep,
			sep_col,
		}
	}
}

pub struct Powerline {
	segments: Vec<Segment>,
	theme: Theme,
}

impl Powerline {
	pub fn new(theme: Theme) -> Powerline {
		Powerline { segments: Vec::new(), theme }
	}

	pub fn add_segments(&mut self, new_segments: Vec<Segment>) {
		for segment in new_segments {
			self.segments.push(segment);
		}
	}
}

#[cfg(feature = "bare-colors")]
impl Powerline {
	pub fn bg_str(&self, color: Color) -> String {
		format!("\x1b[48;5;{}m", self.theme.get(color))
	}

	pub fn fg_str(&self, color: Color) -> String {
		format!("\x1b[38;5;{}m", self.theme.get(color))
	}

	pub fn reset(&self) -> String {
		String::from("\x1b[0m")
	}
}

#[cfg(feature = "bash-colors")]
impl Powerline {
	pub fn bg_str(&self, color: Color) -> String {
		format!("\\[\\e[48;5;{}m\\]", self.theme.get(color))
	}

	pub fn fg_str(&self, color: Color) -> String {
		format!("\\[\\e[38;5;{}m\\]", self.theme.get(color))
	}

	pub fn reset(&self) -> String {
		String::from("\\[\\e[0m\\]")
	}
}

impl fmt::Display for Powerline {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut iter = self.segments.iter().peekable();
		while let Some(seg) = iter.next() {
			write!(
				f,
				"{}{}{}{}{}{}",
				self.fg_str(seg.fg),
				self.bg_str(seg.bg),
				seg.val,
				iter.peek().map(|next| self.bg_str(next.bg)).unwrap_or(self.reset()),
				self.fg_str(seg.sep_col),
				seg.sep
			)?;
		}
		write!(f, "{} ", self.reset())
	}
}
