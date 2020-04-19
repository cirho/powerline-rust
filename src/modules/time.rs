#[cfg(feature = "time")]
use chrono;
use std::{marker::PhantomData};

use super::Module;
use crate::{powerline::Segment, terminal::Color, R};

pub struct Time<S: TimeScheme> {
	time_format: &'static str,
	scheme: PhantomData<S>
}

pub trait TimeScheme {
	const TIME_BG: Color;
	const TIME_FG: Color;
}

impl<S: TimeScheme> Time<S> {
	pub fn new() -> Time<S> {
		Time { time_format: "%H:%M:%S", scheme: PhantomData }
	}

	pub fn with_time_format(time_format: &'static str) -> Time<S> {
		Time { time_format: time_format, scheme: PhantomData }
	}
}

impl<S: TimeScheme> Module for Time<S> {
	fn append_segments(&mut self, segments: &mut Vec<Segment>) -> R<()> {
		let (fg, bg) = (S::TIME_FG, S::TIME_BG);

		let now = chrono::offset::Local::now();
		let value = now.format(self.time_format).to_string();

		segments.push(Segment::simple(format!(" {} ", value), fg, bg));

		Ok(())
	}
}
