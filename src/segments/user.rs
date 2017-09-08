use ::color::Color;
use ::powerline::*;
use ::part::*;

pub struct User;
impl User {
	pub fn new() -> User {
		User {}
	}
}

impl Part for User {
	fn get_segments(self) -> Result<Vec<Segment>, Error> {
		// TODO: Bash only
		// TODO: user special color for root
		Ok(vec![
			Segment::simple(" \\u ", Color::USERNAME_FG, Color::USERNAME_BG)
		])
	}
}
