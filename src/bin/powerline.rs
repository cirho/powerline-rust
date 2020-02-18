extern crate powerline;

use powerline::{modules::*, theme::SimpleTheme};

fn main() -> powerline::R<()> {
	let mut prompt = powerline::Powerline::new();

	prompt.add_module(User::<SimpleTheme>::new())?;
	prompt.add_module(Host::<SimpleTheme>::new())?;
	prompt.add_module(Cwd::<SimpleTheme>::new(45, 4, false))?;
	prompt.add_module(Git::<SimpleTheme>::new())?;
	prompt.add_module(ReadOnly::<SimpleTheme>::new())?;
    prompt.add_module(NewLine::<SimpleTheme>::new())?;
	prompt.add_module(Cmd::<SimpleTheme>::new())?;

	println!("{}", prompt);
	Ok(())
}
