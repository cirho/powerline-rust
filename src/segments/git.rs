use std::{env, marker::PhantomData, path::PathBuf, process::Command, str};

use crate::{part::*, powerline::*, terminal::Color, Error, R};

pub struct GitInfo<S: GitScheme>(PhantomData<S>);

pub struct GitData {
	untracked: u32,
	conflicted: u32,
	non_staged: u32,
	ahead: u32,
	behind: u32,
	staged: u32,
}

pub trait GitScheme {
	const GIT_AHEAD_BG: Color;
	const GIT_AHEAD_FG: Color;
	const GIT_BEHIND_BG: Color;
	const GIT_BEHIND_FG: Color;
	const GIT_STAGED_BG: Color;
	const GIT_STAGED_FG: Color;
	const GIT_NOTSTAGED_BG: Color;
	const GIT_NOTSTAGED_FG: Color;
	const GIT_UNTRACKED_BG: Color;
	const GIT_UNTRACKED_FG: Color;
	const GIT_CONFLICTED_BG: Color;
	const GIT_CONFLICTED_FG: Color;
	const REPO_CLEAN_BG: Color;
	const REPO_CLEAN_FG: Color;
	const REPO_DIRTY_BG: Color;
	const REPO_DIRTY_FG: Color;
}

impl<S: GitScheme> GitInfo<S> {
	pub fn new() -> GitInfo<S> {
		GitInfo(PhantomData)
	}
}

impl GitData {
	fn new() -> GitData {
		GitData {
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
		String::new()
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

fn git_dir_exists() -> Option<PathBuf> {
	let mut git_dir = env::current_dir().unwrap();

	loop {
		git_dir.push(".git/");

		if git_dir.exists() {
			return Some(git_dir);
		}

		git_dir.pop();
		if !git_dir.pop() {
			return None;
		}
	}
}

impl<S: GitScheme> Part for GitInfo<S> {
	fn append_segments(&self, segments: &mut Vec<Segment>) -> R<()> {
		if git_dir_exists().is_none() {
			return Ok(());
		}

		let output = Command::new("git")
			.args(&["status", "--porcelain", "-b"])
			.output()
			.map_err(|e| Error::wrap(e, "Failed to run git"))?
			.stdout;

		if output.is_empty() {
			return Ok(());
		}

		let mut lines = output.split(|x| *x == (b'\n'));

		let branch_line = str::from_utf8(lines.next().ok_or_else(|| Error::from_str("Empty git output"))?)?;
		let mut stats = GitData::new();
		let branch_name = {
			if let Some(branch_name) = get_branch_name(&branch_line) {
				if let Some(pos) = branch_line.find('[') {
					let info = branch_line.get(pos..).unwrap();
					stats.ahead += get_ahead_commits(&info).unwrap_or(0);
					stats.behind += get_behind_commits(&info).unwrap_or(0);
				}
				String::from(branch_name)
			} else {
				get_detached_branch_name()?
			}
		};

		for op in lines.flat_map(|line| line.get(..2)) {
			stats.add_file(str::from_utf8(op)?);
		}

		let (branch_fg, branch_bg) = if stats.is_dirty() {
			(S::REPO_DIRTY_FG, S::REPO_DIRTY_BG)
		} else {
			(S::REPO_CLEAN_FG, S::REPO_CLEAN_BG)
		};

		segments.push(Segment::simple(&format!(" {} ", branch_name), branch_fg, branch_bg));
		{
			let mut add_elem = |count, symbol, fg, bg| {
				if count > 0 {
					let text = format!(" {}{} ", quantity(count), symbol);
					segments.push(Segment::simple(&text, fg, bg));
				}
			};
			add_elem(stats.ahead, '\u{2B06}', S::GIT_AHEAD_FG, S::GIT_AHEAD_BG);
			add_elem(stats.behind, '\u{2B07}', S::GIT_BEHIND_FG, S::GIT_BEHIND_BG);
			add_elem(stats.staged, '\u{2714}', S::GIT_STAGED_FG, S::GIT_STAGED_BG);
			add_elem(stats.non_staged, '\u{270E}', S::GIT_NOTSTAGED_FG, S::GIT_NOTSTAGED_BG);
			add_elem(stats.untracked, '\u{2753}', S::GIT_UNTRACKED_FG, S::GIT_UNTRACKED_BG);
			add_elem(stats.conflicted, '\u{273C}', S::GIT_CONFLICTED_FG, S::GIT_CONFLICTED_BG);
		}
		Ok(())
	}
}
