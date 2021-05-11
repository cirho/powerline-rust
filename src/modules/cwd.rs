use std::marker::PhantomData;
use std::{env, path};

use super::Module;
use crate::{Color, Powerline, Style};

pub struct Cwd<S: CwdScheme> {
    max_length: usize,
    wanted_seg_num: usize,
    resolve_symlinks: bool,
    scheme: PhantomData<S>,
}

pub trait CwdScheme {
    const CWD_FG: Color;
    const PATH_FG: Color;
    const PATH_BG: Color;
    const HOME_FG: Color;
    const HOME_BG: Color;
    const SEPARATOR_FG: Color;
    const CWD_HOME_SYMBOL: &'static str = "~";
}

impl<S: CwdScheme> Cwd<S> {
    pub fn new(max_length: usize, wanted_seg_num: usize, resolve_symlinks: bool) -> Cwd<S> {
        Cwd { max_length, wanted_seg_num, resolve_symlinks, scheme: PhantomData }
    }
}

macro_rules! append_cwd_segments {
    ($powerline:ident, $iter:expr) => {
        for val in $iter {
            $powerline.add_segment(val, Style::special(S::PATH_FG, S::PATH_BG, '\u{E0B1}', S::SEPARATOR_FG));
        }
    };
}

impl<S: CwdScheme> Module for Cwd<S> {
    fn append_segments(&mut self, powerline: &mut Powerline) {
        let current_dir = if self.resolve_symlinks {
            env::current_dir().unwrap()
        } else {
            path::PathBuf::from(env::var("PWD").unwrap())
        };

        let mut cwd = current_dir.to_str().unwrap();

        if cwd == "/" {
            return powerline.add_segment('/', Style::simple(S::PATH_FG, S::PATH_BG));
        }

        if let Some(home_path) = env::home_dir() {
            let home_str = home_path.to_str().unwrap();

            if cwd.starts_with(home_str) {
                powerline.add_segment(S::CWD_HOME_SYMBOL, Style::simple(S::HOME_FG, S::HOME_BG));
                cwd = &cwd[home_str.len()..]
            }
        }

        let depth = cwd.matches('/').count();
        if (cwd.len() > self.max_length as usize) && (depth > self.wanted_seg_num) {
            let left = self.wanted_seg_num / 2;
            let right = self.wanted_seg_num - left;

            let start = cwd.split('/').skip(1).take(left);
            let end = cwd.split('/').skip(depth - right + 1);

            append_cwd_segments!(powerline, start);
            powerline.add_segment('\u{2026}', Style::special(S::PATH_FG, S::PATH_BG, '\u{E0B1}', S::SEPARATOR_FG));
            append_cwd_segments!(powerline, end);
        } else {
            append_cwd_segments!(powerline, cwd.split('/').skip(1));
        };

        if let Some(style) = powerline.last_style_mut() {
            style.sep = '\u{E0B0}';
            style.sep_fg = style.bg.transpose();
        }
    }
}
