use std::fs;
use std::path::Path;
use ::powerline::*;
use ::part::*;
use ::color::Color;

pub struct ReadOnly{ special: &'static str,}

impl ReadOnly {
    pub fn new(special: &'static str ) -> ReadOnly {
        ReadOnly { special }
    }
}

impl Part for ReadOnly {
    fn get_segments(self) -> Result<Vec<Segment>, Error> {
        let metadata = fs::metadata(Path::new("./"))?;
        if !metadata.permissions().readonly() {
            return Ok(vec![Segment::simple(&format!(" {} ", self.special), Color::READONLY_FG, Color::READONLY_BG )]);
        }
        Ok(Vec::new())
    }

}
