use std;

use powerline::Segment;

#[derive(Debug)]
pub struct Error {
    inner: Box<std::error::Error>,
}
impl Error {
    pub fn new(inner: Box<std::error::Error>) -> Error {
        Error { inner }
    }
}
impl<T: std::error::Error + 'static> std::convert::From<T> for Error {
    fn from(e: T) -> Error {
        Error::new(Box::new(e))
    }
}

pub trait Part {
    fn segments(self) -> Result<Vec<Segment>, Error>;
}