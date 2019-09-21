use std::fmt;

use crate::{part::Part, terminal::*};

#[derive(Clone)]
pub struct Segment {
	pub val: String,
	pub fg: FgColor,
	pub bg: BgColor,
	pub sep: char,
	pub sep_col: FgColor,
}

impl Segment {
	pub fn simple<S: Into<String>>(val: S, fg: Color, bg: Color) -> Segment {
		Segment {
			val: val.into(),
			fg: fg.into_fg(),
			bg: bg.into_bg(),
			sep: '\u{E0B0}',
			sep_col: bg.into_fg(),
		}
	}

	pub fn special<S: Into<String>>(val: S, fg: Color, bg: Color, sep: char, sep_col: Color) -> Segment {
		Segment {
			val: val.into(),
			fg: fg.into_fg(),
			bg: bg.into_bg(),
			sep,
			sep_col: sep_col.into_fg(),
		}
	}
}

pub struct Powerline {
	segments: Vec<Segment>,
}

impl Powerline {
	pub fn new() -> Powerline {
		Powerline { segments: Vec::new() }
	}

	pub fn add_part(&mut self, mut part: impl Part) {
		part.append_segments(&mut self.segments).expect("part failed")
	}

	pub fn add_segments(&mut self, new_segments: Vec<Segment>) {
		self.segments.extend(new_segments);
	}
}

impl fmt::Display for Powerline {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut iter = self.segments.iter().peekable();
		while let Some(seg) = iter.next() {
			if let Some(next) = iter.peek() {
				write!(f, "{}{}{}{}{}{}", seg.fg, seg.bg, seg.val, next.bg, seg.sep_col, seg.sep)?;
			} else {
				write!(f, "{}{}{}{}{}{}", seg.fg, seg.bg, seg.val, Reset, seg.sep_col, seg.sep)?;
			}
		}
		write!(f, "{} ", Reset)
	}
}
