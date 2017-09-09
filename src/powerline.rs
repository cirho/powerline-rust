use color::Color;
use themes::Theme;
use std::fmt;

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

pub struct Powerline { segments : Vec<Segment>, theme: Theme }

impl Powerline {
    pub fn new(theme: Theme) -> Powerline { Powerline { segments: Vec::new(), theme } }
    pub fn add_segments(&mut self, new_segments: Vec<Segment>) {
        for segment in new_segments {
            self.segments.push(segment);
        }
    }

    fn fg_str(&self, color: Color) -> String { format!("\\[\\e[38;5;{}m\\]", self.theme.get(color)) }
    fn bg_str(&self, color: Color) -> String { format!("\\[\\e[48;5;{}m\\]", self.theme.get(color)) }
    fn reset(&self) -> String { String::from("\\[\\e[0m\\]") }
}

impl fmt::Display for Powerline {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let size = self.segments.len();
        for idx in  0..(size) {
            let seg = &self.segments[idx];
            let next_col = if idx != size - 1 {
                self.bg_str(self.segments[idx+1].bg)
            } else {
                self.reset()
            };
            write!(f, "{}{}{}{}{}{}", self.fg_str(seg.fg), self.bg_str(seg.bg), seg.val, next_col, self.fg_str(seg.sep_col), seg.sep)?;
        }
        write!(f, "{} ", self.reset())?;
        Ok(())
    }
}
