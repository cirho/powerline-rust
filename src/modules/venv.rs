use std::env;
use std::marker::PhantomData;
use std::path::Path;

use super::Module;
use crate::{Color, Powerline, Style};

pub struct VirtualEnv<S: VirtualEnvScheme> {
    scheme: PhantomData<S>,
}

pub trait VirtualEnvScheme {
    const PYVENV_FG: Color;
    const PYVENV_BG: Color;
}

impl<S: VirtualEnvScheme> VirtualEnv<S> {
    pub fn new() -> VirtualEnv<S> {
        VirtualEnv { scheme: PhantomData }
    }
}

impl<S: VirtualEnvScheme> Module for VirtualEnv<S> {
    fn append_segments(&mut self, powerline: &mut Powerline) {
        let venv =
            env::var("VIRTUAL_ENV").or_else(|_| env::var("CONDA_ENV_PATH")).or_else(|_| env::var("CONDA_DEFAULT_ENV"));

        if let Ok(venv_path) = venv {
            // file_name is always some, because env variable is a valid directory path.
            let venv_name = Path::new(&venv_path).file_name().unwrap().to_string_lossy();

            powerline.add_segment(venv_name, Style::simple(S::PYVENV_FG, S::PYVENV_BG))
        }
    }
}
