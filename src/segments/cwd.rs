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
    match env::home_dir() {
        Some(home_path) => {
            let home = pb_to_str(home_path);
            match cwd.find(&home) {
                    Some(pos) => {
                    cwd_slice = cwd.get((pos + home.len() )..).unwrap();
                    prompt.add_segment(Segment::simple(&format!(" {} ", special), Color::HOME_FG, Color::HOME_BG) )
                    },
                None => {},
            }
        },
        None => {},
    }
    let path: Vec<&str> = cwd_slice.split("/").skip(1).collect();
    let size = path.len();
    for idx in 0..size {
        let el = &path[idx];
        if idx != size - 1 {
            prompt.add_segment(Segment::special(&format!(" {} ", el), Color::PATH_FG, Color::PATH_BG, '\u{E0B1}', Color::SEPARATOR_FG ) );
        }
        else {
            prompt.add_segment(Segment::simple(&format!(" {} ", el), Color::PATH_FG, Color::PATH_BG) );
        }
    }
}
