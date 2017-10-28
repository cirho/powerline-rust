use std::env;
use ::color::Color;
use ::powerline::*;
use ::part::*;
use users;

pub struct Cmd {
    normal: &'static str,
    root: &'static str,
}
impl Cmd {
    pub fn new(normal: &'static str, root: &'static str) -> Cmd {
        Cmd { normal, root }
    }
}

impl Part for Cmd {
    fn get_segments(self) -> Result<Vec<Segment>, Error> {
        let status = env::args().nth(1).ok_or(Error::from_str("You should pass $? as argument"))?;
        let (fg, bg) = if status != "0" {
            (Color::CMD_FAILED_FG, Color::CMD_FAILED_BG)
        } else {
            (Color::CMD_PASSED_FG, Color::CMD_PASSED_BG)
        };
        let is_root = users::get_current_uid() == 0;
        let special = if is_root { self.root } else { self.normal };
        Ok(vec![Segment::simple(&format!(" {} ", special), fg, bg)])
    }
}
