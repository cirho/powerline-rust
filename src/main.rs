#![feature(iterator_for_each)]

extern crate regex;

mod powerline;
mod segments;
mod color;

use segments::*;
fn main() {
    let mut prompt = powerline::Powerline::new();
    cwd::add_segment(&mut prompt, "λ");
    git::add_segment(&mut prompt);
    cmd::add_segment(&mut prompt, "\\$");


    println!("{}", prompt);

}
