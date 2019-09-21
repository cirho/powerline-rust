extern crate powerline;

use powerline::{segments::*, theme::SimpleTheme};

fn main() {
	let mut prompt = powerline::Powerline::new();

	prompt.add_part(user::User::<SimpleTheme>::new());
	prompt.add_part(host::Host::<SimpleTheme>::new());
	prompt.add_part(cwd::Cwd::<SimpleTheme>::new(45, 4, false));
	prompt.add_part(git::Git::<SimpleTheme>::with_file_cache("/tmp/powerline").unwrap());
	prompt.add_part(readonly::ReadOnly::<SimpleTheme>::new());
	prompt.add_part(cmd::Cmd::<SimpleTheme>::new());

	println!("{}", prompt);
}
