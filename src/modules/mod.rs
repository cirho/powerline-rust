use crate::{powerline::Segment, R};

mod cmd;
mod cwd;
mod git;
mod host;
mod readonly;
mod time;
mod user;

pub use cmd::{Cmd, CmdScheme};
pub use cwd::{Cwd, CwdScheme};
pub use git::{Git, GitScheme};
pub use host::{Host, HostScheme};
pub use readonly::{ReadOnly, ReadOnlyScheme};
pub use time::{Time, TimeScheme};
pub use user::{User, UserScheme};

pub trait Module: Sized {
	fn append_segments(&mut self, segments: &mut Vec<Segment>) -> R<()>;

	#[inline]
	fn into_segments(mut self) -> R<Vec<Segment>> {
		self.get_segments()
	}

	#[inline]
	fn get_segments(&mut self) -> R<Vec<Segment>> {
		let mut vec = Vec::new();

		self.append_segments(&mut vec).map(|_| vec)
	}
}
