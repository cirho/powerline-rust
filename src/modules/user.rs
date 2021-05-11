use std::marker::PhantomData;

use super::Module;
use crate::{utils, Color, Powerline, Style};

pub struct User<S: UserScheme> {
	show_on_local: bool,
	scheme: PhantomData<S>,
}

pub trait UserScheme {
	const USERNAME_ROOT_BG: Color;
	const USERNAME_BG: Color;
	const USERNAME_FG: Color;
}

impl<S: UserScheme> User<S> {
	pub fn new() -> User<S> {
		User { show_on_local: true, scheme: PhantomData }
	}

	pub fn show_on_remote_shell() -> User<S> {
		User { show_on_local: false, scheme: PhantomData }
	}
}

impl<S: UserScheme> Module for User<S> {
	fn append_segments(&mut self, powerline: &mut Powerline) {
		if self.show_on_local || utils::is_remote_shell() {
			let user = users::get_user_by_uid(users::get_current_uid()).unwrap();
			let bg = if user.uid() == 0 { S::USERNAME_ROOT_BG } else { S::USERNAME_BG };

			powerline.add_segment(user.name().to_str().unwrap(), Style::simple(S::USERNAME_FG, bg));
		}
	}
}
