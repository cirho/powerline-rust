use crate::{error::Error, powerline::Segment};

pub trait Part {
	fn get_segments(self) -> Result<Vec<Segment>, Error>;
}
