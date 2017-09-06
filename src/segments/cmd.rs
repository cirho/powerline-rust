use std::env;
use ::color::Color;
use ::powerline::*;
use ::part::*;

pub struct Cmd {
    special: &'static str,
}
impl Cmd {
    pub fn new(special: &'static str) -> Cmd {
        Cmd { special }
    }
}

impl Part for Cmd {
    fn segments(self) -> Result<Vec<Segment>, Error> {
        let status = env::args().nth(1).ok_or(Error::from_str("You should pass $? as argument"))?;
        let (fg, bg) = if status != "0" {
            (Color::CMD_FAILED_FG, Color::CMD_FAILED_BG)
        } else {
            (Color::CMD_PASSED_FG, Color::CMD_PASSED_BG)
        };
        Ok(vec![Segment::simple(&format!(" {} ", self.special), fg, bg)])
    }
}
