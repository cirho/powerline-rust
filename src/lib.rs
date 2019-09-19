mod error;
pub mod part;
pub mod powerline;
pub mod segments;
pub mod terminal;
pub mod theme;

pub type R<T> = Result<T, error::Error>;
pub use error::Error;
pub use powerline::*;
