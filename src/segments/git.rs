use std::{env, process::Command, str};

use crate::{color::Color, part::*, powerline::*, Error};

pub struct GitInfo {
	untracked: u32,
	conflicted: u32,
	non_staged: u32,
	ahead: u32,
	behind: u32,
	staged: u32,
}

impl GitInfo {
	pub fn new() -> GitInfo {
		GitInfo {
			untracked: 0,
			conflicted: 0,
			non_staged: 0,
			staged: 0,
			ahead: 0,
			behind: 0,
		}
	}

	fn is_dirty(&self) -> bool {
		(self.untracked + self.conflicted + self.staged + self.non_staged) > 0
	}

	fn add_file(&mut self, begin: &str) {
		match begin {
			"??" => self.untracked += 1,
			"DD" => self.conflicted += 1,
			"AU" => self.conflicted += 1,
			"UD" => self.conflicted += 1,
			"UA" => self.conflicted += 1,
			"UU" => self.conflicted += 1,
			"DU" => self.conflicted += 1,
			"AA" => self.conflicted += 1,
			_ => {
				let mut chars = begin.chars();
				let a = chars.next().expect("invalid file status");
				let b = chars.next().expect("invalid file status");
				if b != ' ' {
					self.non_staged += 1;
				}
				if a != ' ' {
					self.staged += 1;
				}
			},
		};
	}
}

fn get_detached_branch_name() -> Result<String, Error> {
	let child = Command::new("git")
		.args(&["describe", "--tags", "--always"])
		.output()
		.map_err(|e| Error::wrap(e, "Failed to run git"))?;
	Ok(if child.status.success() {
		let branch = str::from_utf8(&child.stdout)?.split('\n').next().ok_or_else(|| Error::from_str("Empty git output"))?;
		format!("\u{2693}{}", branch)
	} else {
		String::from("Big Bang")
	})
}

fn quantity(val: u32) -> String {
	if val > 1 {
		format!("{}", val)
	} else {
		String::from("")
	}
}

fn get_first_number(s: &str) -> u32 {
	s.chars().take_while(|x| x.is_digit(10)).flat_map(|x| x.to_digit(10)).fold(0, |acc, x| 10 * acc + x)
}

fn get_ahead_commits(s: &str) -> Option<u32> {
	s.find("ahead").map(|pos| {
		let start = pos + 6;
		let rest = s.get(start..).unwrap();
		get_first_number(rest)
	})
}

fn get_behind_commits(s: &str) -> Option<u32> {
	s.find("behind").map(|pos| {
		let start = pos + 7;
		let rest = s.get(start..).unwrap();
		get_first_number(rest)
	})
}

fn get_branch_name(s: &str) -> Option<&str> {
	if let Some(rest) = s.get(3..) {
		let mut end: usize = 0;
		if let Some(pos) = rest.find("...") {
			end = pos
		} else {
			let mut text = rest.chars();
			while let Some(c) = text.next() {
				end += 1;
				if c.is_whitespace() {
					if Some('[') != text.next() {
						return None;
					}
					break;
				}
			}
		}
		rest.get(..end)
	} else {
		None
	}
}

fn git_dir_exists() -> bool {
	let mut git_dir = env::current_dir().unwrap();

	loop {
		git_dir.push(".git/");

		if git_dir.exists() {
			return true;
		}

		git_dir.pop();
		if !git_dir.pop() {
			return false;
		}
	}
}

impl Part for GitInfo {
	fn get_segments(mut self) -> Result<Vec<Segment>, Error> {
		if !git_dir_exists() {
			return Ok(vec![]);
		}

		let output = Command::new("git")
			.args(&["status", "--porcelain", "-b"])
			.output()
			.map_err(|e| Error::wrap(e, "Failed to run git"))?
			.stdout;

		if output.is_empty() {
			return Ok(vec![]);
		}

		let mut lines = output.split(|x| *x == (b'\n'));

		let branch_line = str::from_utf8(lines.next().ok_or_else(|| Error::from_str("Empty git output"))?)?;

		let branch_name = {
			if let Some(branch_name) = get_branch_name(&branch_line) {
				if let Some(pos) = branch_line.find('[') {
					let info = branch_line.get(pos..).unwrap();
					self.ahead += get_ahead_commits(&info).unwrap_or(0);
					self.behind += get_behind_commits(&info).unwrap_or(0);
				}
				String::from(branch_name)
			} else {
				get_detached_branch_name()?
			}
		};

		for op in lines.flat_map(|line| line.get(..2)) {
			self.add_file(str::from_utf8(op)?);
		}

		let (branch_fg, branch_bg) = if self.is_dirty() {
			(Color::REPO_DIRTY_FG, Color::REPO_DIRTY_BG)
		} else {
			(Color::REPO_CLEAN_FG, Color::REPO_CLEAN_BG)
		};

		let mut segments = Vec::new();
		segments.push(Segment::simple(&format!(" {} ", branch_name), branch_fg, branch_bg));
		{
			let mut add_elem = |count, symbol, fg, bg| {
				if count > 0 {
					let text = format!(" {}{} ", quantity(count), symbol);
					segments.push(Segment::simple(&text, fg, bg));
				}
			};
			add_elem(self.ahead, '\u{2B06}', Color::GIT_AHEAD_FG, Color::GIT_AHEAD_BG);
			add_elem(self.behind, '\u{2B07}', Color::GIT_BEHIND_FG, Color::GIT_BEHIND_BG);
			add_elem(self.staged, '\u{2714}', Color::GIT_STAGED_FG, Color::GIT_STAGED_BG);
			add_elem(self.non_staged, '\u{270E}', Color::GIT_NOTSTAGED_FG, Color::GIT_NOTSTAGED_BG);
			add_elem(self.untracked, '\u{2753}', Color::GIT_UNTRACKED_FG, Color::GIT_UNTRACKED_BG);
			add_elem(self.conflicted, '\u{273C}', Color::GIT_CONFLICTED_FG, Color::GIT_CONFLICTED_BG);
		}
		Ok(segments)
	}
}
