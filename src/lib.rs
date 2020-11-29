pub mod modules;
pub mod powerline;
pub mod terminal;
pub mod theme;
pub(crate) mod utils;

pub type R<T> = Result<T, Box<dyn std::error::Error>>;
pub use crate::powerline::{Powerline, Segment};
