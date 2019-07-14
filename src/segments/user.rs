use crate::{color::Color, part::*, powerline::*, Error};

pub struct User;

impl User {
	pub fn new() -> User {
		User
	}
}

impl Part for User {
	fn get_segments(self) -> Result<Vec<Segment>, Error> {
		// TODO: Bash only
		let is_root = users::get_current_uid() == 0;
		let bg = if is_root { Color::USERNAME_ROOT_BG } else { Color::USERNAME_BG };
		Ok(vec![Segment::simple(
			&format!(" {} ", users::get_current_username().unwrap().to_str().unwrap()),
			Color::USERNAME_FG,
			bg,
		)])
	}
}
