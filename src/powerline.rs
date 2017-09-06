use color::Color;
use std::fmt;
use part::*;

pub struct Segment {
    pub val: String,
    pub fg: Color,
    pub bg: Color,
    pub sep: char,
    pub sep_col: Color,
}

impl Segment {
    pub fn simple(val: &str, fg: Color, bg: Color) -> Segment {
        Segment {val: val.to_owned(), fg: fg, bg: bg.clone(), sep: '\u{E0B0}', sep_col: bg}
    }
    pub fn special(val: &str, fg: Color, bg: Color, sep: char, sep_col: Color) -> Segment {
        Segment {val: val.to_owned(), fg: fg, bg: bg, sep: sep, sep_col: sep_col}
    }
}

pub struct Powerline { segments : Vec<Segment> }

impl Powerline {
    pub fn new() -> Powerline { Powerline { segments: Vec::new() } }
    pub fn add_seg<T: Part>(&mut self, seg: T) -> Result<(), Error> {
        for segment in seg.segments()? {
            self.segments.push(segment);
        }
        Ok(())
    }
}


impl fmt::Display for Powerline {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let size = self.segments.len();
        for idx in  0..(size) {
            let seg = &self.segments[idx];
            let mut next_col = Color::reset();
            if idx != size - 1 {
                next_col = self.segments[idx + 1].bg.bg_str();
            }
            write!(f, "{}{}{}{}{}{}",seg.fg.fg_str(), seg.bg.bg_str(), seg.val, next_col, seg.sep_col.fg_str(), seg.sep)?;
        }
        write!(f, "{} ", Color::reset())?;
        Ok(())
    }
}
