use crate::{error::Error, powerline::Segment};

pub trait Part: Sized {
	fn append_segments(&self, segments: &mut Vec<Segment>) -> Result<(), Error>;
	#[inline]
	fn into_segments(self) -> Result<Vec<Segment>, Error> {
		self.get_segments()
	}
	#[inline]
	fn get_segments(&self) -> Result<Vec<Segment>, Error> {
		let mut vec = Vec::new();
		self.append_segments(&mut vec).map(|_| vec)
	}
}
