use powerline::modules::*;
use powerline::theme::SimpleTheme;

fn main() {
    let mut prompt = powerline::Powerline::new();

    prompt.add_module(Cwd::<SimpleTheme>::new(45, 4, false));
    prompt.add_module(Git::<SimpleTheme>::new());
    prompt.add_module(ReadOnly::<SimpleTheme>::new());
    prompt.add_module(Cmd::<SimpleTheme>::new());

    println!("{} ", prompt);
}
