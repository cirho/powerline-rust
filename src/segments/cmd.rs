use std::env;
use ::color::Color;
use ::powerline::*;

pub fn add_segment(prompt : &mut Powerline, special: &str) {
    let status = match  env::args().nth(1){
        Some(s) => s,
        None => panic!("You should pass $? as argument")
    };
    let mut bg = Color::CMD_PASSED_BG;
    let mut fg = Color::CMD_PASSED_FG;
    if status != "0" {
        bg = Color::CMD_FAILED_BG;
        fg = Color::CMD_FAILED_FG;
    }

    prompt.add_segment(Segment::simple(&format!(" {} ", special), fg, bg))

}
