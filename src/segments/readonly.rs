use std::fs;
use std::path::Path;
use ::powerline::*;
use ::part::*;
use ::color::Color;
use std::ffi::CString;
use libc;

pub struct ReadOnly{ special: &'static str,}

impl ReadOnly {
    pub fn new(special: &'static str ) -> ReadOnly {
        ReadOnly { special }
    }
}

impl Part for ReadOnly {
    fn get_segments(self) -> Result<Vec<Segment>, Error> {
        let readonly = unsafe{
            let path = CString::new("./")?;
            libc::access(path.as_ptr(), libc::W_OK) != 0
        };
        if readonly {
            return Ok(vec![Segment::simple(&format!(" {} ", self.special), Color::READONLY_FG, Color::READONLY_BG )]);
        }
        Ok(Vec::new())
    }

}
