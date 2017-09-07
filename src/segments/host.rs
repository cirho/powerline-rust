pub use ::color::Color;
pub use ::powerline::*;
pub use ::part::*;

pub struct Host;
impl Host {
	pub fn new() -> Host {
		Host {}
	}
}

impl Part for Host {
	fn get_segments(self) -> Result<Vec<Segment>, Error> {
		// TODO: Bash only
		Ok(vec![
			Segment::simple(" \\h ", Color::HOSTNAME_FG, Color::HOSTNAME_BG)
		])
	}
}
