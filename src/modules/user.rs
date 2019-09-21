use std::marker::PhantomData;

use super::Module;
use crate::{Segment, terminal::Color, R};

pub struct User<S: UserScheme>(PhantomData<S>);
pub trait UserScheme {
	const USERNAME_ROOT_BG: Color;
	const USERNAME_BG: Color;
	const USERNAME_FG: Color;
}

impl<S: UserScheme> User<S> {
	pub fn new() -> User<S> {
		User(PhantomData)
	}
}

impl<S: UserScheme> Module for User<S> {
	fn append_segments(&mut self, segments: &mut Vec<Segment>) -> R<()> {
		let user = users::get_user_by_uid(users::get_current_uid()).unwrap();
		let bg = if user.uid() == 0 { S::USERNAME_ROOT_BG } else { S::USERNAME_BG };

		segments.push(Segment::simple(format!(" {} ", user.name().to_str().unwrap()), S::USERNAME_FG, bg));
		Ok(())
	}
}
