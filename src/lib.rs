pub mod modules;
pub mod powerline;
pub mod terminal;
pub mod theme;

pub(crate) mod utils;

pub use crate::{
	powerline::{Powerline, Style}, terminal::Color
};
