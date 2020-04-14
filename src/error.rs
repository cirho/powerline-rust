#[derive(Debug)]
pub struct Error {
	inner: Option<Box<dyn std::error::Error>>,
	desc: Option<&'static str>,
}

impl Error {
	pub fn from_err(inner: Box<dyn std::error::Error>) -> Error {
		Error { inner: Some(inner), desc: None }
	}

	pub fn from_str(s: &'static str) -> Error {
		Error { inner: None, desc: Some(s) }
	}

	pub fn wrap<E: std::error::Error + 'static>(inner: E, desc: &'static str) -> Error {
		Error { inner: Some(Box::new(inner)), desc: Some(desc) }
	}
}

impl<T: std::error::Error + 'static> std::convert::From<T> for Error {
	fn from(e: T) -> Error {
		Error::from_err(Box::new(e))
	}
}
