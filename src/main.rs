extern crate regex;

mod powerline;
mod segments;
mod part;
mod color;

use segments::*;
fn main() {
    let mut prompt = powerline::Powerline::new();
    prompt.add_seg(cwd::Cwd::new("λ")).expect("Failed seg: Cwd");
    prompt.add_seg(git::GitInfo::new()).expect("Failed seg: Git");
    prompt.add_seg(cmd::Cmd::new("\\$")).expect("Failed seg: Cmd");


    println!("{}", prompt);

}
