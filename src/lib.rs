mod error;
pub mod modules;
pub mod powerline;
pub mod terminal;
pub mod theme;

pub type R<T> = Result<T, error::Error>;
pub use error::Error;
pub use powerline::{Powerline, Segment};
