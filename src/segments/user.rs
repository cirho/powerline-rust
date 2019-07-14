use crate::{color::Color, part::*, powerline::*, Error};

pub struct User;

impl User {
	pub fn new() -> User {
		User
	}
}

impl Part for User {
	fn get_segments(self) -> Result<Vec<Segment>, Error> {
		let user = users::get_user_by_uid(users::get_current_uid()).unwrap();
		let bg = if user.uid() == 0 { Color::USERNAME_ROOT_BG } else { Color::USERNAME_BG };

		Ok(vec![Segment::simple(&format!(" {} ", user.name().to_str().unwrap()), Color::USERNAME_FG, bg)])
	}
}
