pub use color::Color;
pub use part::*;
pub use powerline::*;

pub struct Host;
impl Host {
	pub fn new() -> Host {
		Host {}
	}
}

impl Part for Host {
	fn get_segments(self) -> Result<Vec<Segment>, Error> {
		// TODO: Bash only
		Ok(vec![Segment::simple(" \\h ", Color::HOSTNAME_FG, Color::HOSTNAME_BG)])
	}
}
