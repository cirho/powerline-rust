mod powerline;
mod segments;
mod color;

use segments::*;
fn main() {
    let mut prompt = powerline::Powerline::new();
    cwd::add_segment(&mut prompt);
    cmd::add_segment(&mut prompt);

    println!("{}", prompt);

}
