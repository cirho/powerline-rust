extern crate powerline;

use powerline::{part::*, segments::*};

fn main() {
	let mut prompt = powerline::Powerline::new(powerline::theme::DEFAULT_THEME);
	prompt.add_segments(user::User::new().get_segments().expect("Failed seg: User"));
	prompt.add_segments(host::Host::new().get_segments().expect("Failed seg: Host"));
	prompt.add_segments(cwd::Cwd::new("~", 45, 4, false).get_segments().expect("Failed seg: Cwd"));
	prompt.add_segments(git::GitInfo::new().get_segments().expect("Failed seg: Git"));
	prompt.add_segments(readonly::ReadOnly::new("").get_segments().expect("Failed seg: ReadOnly"));
	prompt.add_segments(cmd::Cmd::new("$", "#").get_segments().expect("Failed seg: Cmd"));
	println!("{}", prompt);
}
