use std::path;
use std::env;
use ::powerline::*;
use ::color::Color;

fn pb_to_str(path: path::PathBuf) -> String {
    path.to_str().unwrap().to_string()
}

pub fn add_segment(prompt : &mut Powerline, special: &str) {
    let cwd = pb_to_str(env::current_dir().unwrap());
    let mut cwd_slice = cwd.get(0..).unwrap();
    if let Some(home_path) = env::home_dir() {
        let home = pb_to_str(home_path);
        if let Some(pos) = cwd.find(&home) {
            cwd_slice = cwd.get((pos + home.len() )..).unwrap();
            prompt.add_segment(Segment::simple(&format!(" {} ", special), Color::HOME_FG, Color::HOME_BG) )
        }
    }

    let mut counter = 0;
    for val in cwd_slice.split("/").skip(1) {
        prompt.add_segment(Segment::special(&format!(" {} ", val), Color::PATH_FG, Color::PATH_BG, '\u{E0B1}', Color::SEPARATOR_FG ) );
        counter += 1
    }
    if counter > 0 {
        let last = prompt.last_segment_mut().unwrap();
        if last.val == "  " { last.val = " / ".to_owned()}
        last.sep = '\u{E0B0}';
        last.sep_col = last.bg;
    }
}
