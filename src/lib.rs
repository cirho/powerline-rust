pub mod modules;
pub mod powerline;
pub mod terminal;
pub mod theme;
pub(crate) mod utils;

pub type R<T> = anyhow::Result<T>;
pub use crate::powerline::{Powerline, Segment};
