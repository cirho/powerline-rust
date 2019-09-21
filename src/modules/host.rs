use std::marker::PhantomData;

use super::Module;
use crate::{Segment, terminal::Color, R};

pub struct Host<S: HostScheme>(PhantomData<S>);

pub trait HostScheme {
	const HOSTNAME_FG: Color;
	const HOSTNAME_BG: Color;
}
impl<S: HostScheme> Host<S> {
	pub fn new() -> Host<S> {
		Host(PhantomData)
	}
}

impl<S: HostScheme> Module for Host<S> {
	fn append_segments(&mut self, segments: &mut Vec<Segment>) -> R<()> {
		segments.push(Segment::simple(" \\h ", S::HOSTNAME_FG, S::HOSTNAME_BG));
		Ok(())
	}
}
