extern crate powerline;

use powerline::{segments::*, theme::SimpleTheme};

fn main() {
	let mut prompt = powerline::Powerline::new();

	prompt.add_part(cwd::Cwd::<SimpleTheme>::new(45, 4, false));
	prompt.add_part(git::GitInfo::<SimpleTheme>::new());
	prompt.add_part(readonly::ReadOnly::<SimpleTheme>::new());
	prompt.add_part(cmd::Cmd::<SimpleTheme>::new());

	println!("{}", prompt);
}
