use std::path;
use std::env;
use ::powerline::*;
use ::color::Color;
use ::part::*;

pub struct Cwd {
    special: &'static str,
}
impl Cwd {
    pub fn new(special: &'static str) -> Cwd {
        Cwd { special }
    }
}

fn pb_to_str(path: path::PathBuf) -> String {
    path.to_str().unwrap().to_string()
}

impl Part for Cwd {
fn segments(self) -> Result<Vec<Segment>, Error> {
    let cwd = pb_to_str(env::current_dir().unwrap());
    let mut cwd_slice = cwd.get(0..).unwrap();
    let mut results = Vec::new();
    if let Some(home_path) = env::home_dir() {
        let home = pb_to_str(home_path);
        if let Some(pos) = cwd.find(&home) {
            cwd_slice = cwd.get((pos + home.len() )..).unwrap();
            results.push(Segment::simple(&format!(" {} ", self.special), Color::HOME_FG, Color::HOME_BG) )
        }
    }

    let mut counter = 0;
    for val in cwd_slice.split("/").skip(1) {
        results.push(Segment::special(&format!(" {} ", val), Color::PATH_FG, Color::PATH_BG, '\u{E0B1}', Color::SEPARATOR_FG ) );
        counter += 1
    }
    if counter > 0 {
        let last = results.last_mut().unwrap();
        if last.val == "  " { last.val = " / ".to_owned()}
        last.sep = '\u{E0B0}';
        last.sep_col = last.bg;
    }
    Ok(results)
}
}
