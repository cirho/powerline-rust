#[derive(Clone, Copy)]
pub struct Color(pub u8);
#[derive(Clone, Copy)]
pub struct BgColor(u8);
#[derive(Clone, Copy)]
pub struct FgColor(u8);
pub struct Reset;

impl Color {
	pub fn to_u8(&self) -> u8 {
		self.0
	}

	pub fn into_bg(self) -> BgColor {
		BgColor(self.0)
	}

	pub fn into_fg(self) -> FgColor {
		FgColor(self.0)
	}

	pub fn from_u8(val: u8) -> Color {
		Color(val)
	}
}

impl FgColor {
	pub fn transpose(self) -> BgColor {
		BgColor(self.0)
	}
}

impl BgColor {
	pub fn transpose(self) -> FgColor {
		FgColor(self.0)
	}
}

impl std::fmt::Display for BgColor {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		#[cfg(feature = "bash-shell")]
		return write!(f, r#"\[\e[48;5;{}m\]"#, self.0);

		#[cfg(feature = "bare-shell")]
		return write!(f, "\x1b[48;5;{}m", self.0);

		#[cfg(feature = "zsh-shell")]
		return write!(f, "%{{\x1b[48;5;{}m%}}", self.0);
	}
}

impl std::fmt::Display for FgColor {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		#[cfg(feature = "bash-shell")]
		return write!(f, r#"\[\e[38;5;{}m\]"#, self.0);

		#[cfg(feature = "bare-shell")]
		return write!(f, "\x1b[38;5;{}m", self.0);

		#[cfg(feature = "zsh-shell")]
		return write!(f, "%{{\x1b[38;5;{}m%}}", self.0);
	}
}

impl std::fmt::Display for Reset {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		#[cfg(feature = "bare-shell")]
		return f.write_str("\x1b[0m");

		#[cfg(feature = "bash-shell")]
		return f.write_str(r#"\[\e[0m\]"#);

		#[cfg(feature = "zsh-shell")]
		return f.write_str("%{\x1b[39m%}%{\x1b[49m%}");
	}
}
