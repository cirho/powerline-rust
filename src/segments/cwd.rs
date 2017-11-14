use std::path;
use std::env;
use ::powerline::*;
use ::color::Color;
use ::part::*;

pub struct Cwd {
    special: &'static str,
    max_length: i32,
    wanted_seg_num: i32,
}

impl Cwd {
    pub fn new(special: &'static str, max_length: i32, wanted_seg_num: i32) -> Cwd {
        Cwd { special, max_length, wanted_seg_num }
    }
}

fn pb_to_str(path: path::PathBuf) -> Result<String, Error> {
    Ok(path.to_str().ok_or(Error::from_str("Path is not valid UTF-8"))?.to_string())
}

impl Part for Cwd {
    fn get_segments(self) -> Result<Vec<Segment>, Error> {
        let cwd = pb_to_str(env::current_dir()?)?;
        let mut segments = Vec::new();
        let cwd_slice = if let Some(home_path) = env::home_dir() {
            let home = pb_to_str(home_path)?;
            if let Some(pos) = cwd.find(&home) {
                segments.push(Segment::simple(&format!(" {} ", self.special), Color::HOME_FG, Color::HOME_BG) );
                &cwd[pos+home.len()..]
            } else {
                cwd.as_str()
            }
        } else {
            cwd.as_str()
        };
        let dots = "\u{2026}";
        let depth: i32 = cwd_slice.matches("/").count() as i32- 1;
        let iter: Vec<&str> = if (cwd_slice.len() > self.max_length as usize) && (depth > self.wanted_seg_num) {
            let left = self.wanted_seg_num / 2;
            let right = self.wanted_seg_num - left;
            let start = cwd_slice.split("/").skip(1).take(left as usize);
            let end = cwd_slice.split("/").skip((depth - right + 2) as usize);
            start.chain(dots.split("/")).chain(end).collect()
        }
        else {
            cwd_slice.split("/").skip(1).collect()
        };

        for val in iter {
            segments.push(Segment::special(&format!(" {} ", val), Color::PATH_FG, Color::PATH_BG, '\u{E0B1}', Color::SEPARATOR_FG ) );
        }
        if let Some(last) = segments.last_mut() {
            if last.val == "  " { last.val = " / ".to_owned()}
            last.fg = Color::CWD_FG;
            last.sep = '\u{E0B0}';
            last.sep_col = last.bg;
        }

        Ok(segments)
    }
}
