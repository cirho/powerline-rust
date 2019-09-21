use std::{
	collections::HashMap, env, marker::PhantomData, path, path::{Path, PathBuf}, time::SystemTime
};

use super::Module;
use crate::{Segment, terminal::Color, Error, R};

mod internal;

pub struct Git<S> {
	cache: Option<HashMap<String, (GitStats, u64)>>,
	#[cfg(feature = "git-file-cache")]
	file_path: Option<PathBuf>,
	scheme: PhantomData<S>,
}

#[cfg_attr(feature = "git-file-cache", derive(miniserde::Deserialize, miniserde::Serialize))]
#[derive(Clone)]
pub struct GitStats {
	pub untracked: u32,
	pub conflicted: u32,
	pub non_staged: u32,
	pub ahead: u32,
	pub behind: u32,
	pub staged: u32,
	pub branch_name: String,
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
	const GIT_REPO_CLEAN_BG: Color;
	const GIT_REPO_CLEAN_FG: Color;
	const GIT_REPO_DIRTY_BG: Color;
	const GIT_REPO_DIRTY_FG: Color;
}

impl<S: GitScheme> Git<S> {
	pub fn new() -> Git<S> {
		Git::with_memory_cache()
	}

	#[cfg(feature = "git-file-cache")]
	pub fn with_file_cache<P: AsRef<Path>>(path: P) -> R<Git<S>> {
		Ok(Git {
			cache: None,
			#[cfg(feature = "git-file-cache")]
			file_path: Some(path.as_ref().to_path_buf()),
			scheme: PhantomData,
		})
	}

	pub fn with_memory_cache() -> Git<S> {
		Git {
			cache: Some(HashMap::new()),
			#[cfg(feature = "git-file-cache")]
			file_path: None,
			scheme: PhantomData,
		}
	}

	fn lazy_cache_mut(&mut self) -> R<Option<&mut HashMap<String, (GitStats, u64)>>> {
		match &self.cache {
			Some(_) => Ok(self.cache.as_mut()),
			#[cfg(feature = "git-file-cache")]
			None if self.file_path.is_some() => {
				let path = self.file_path.as_ref().unwrap();
				self.cache = Some(if path.exists() {
					miniserde::json::from_str(&std::fs::read_to_string(path)?)?
				} else {
					HashMap::new()
				});
				Ok(self.cache.as_mut())
			},
			_ => Ok(None),
		}
	}

	pub fn get_git_data(&mut self, path: PathBuf) -> R<GitStats> {
		if let Some(ref mut cache) = self.lazy_cache_mut()? {
			let to_seconds = |time: SystemTime| time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

			if let Some((cached_stats, cached_time)) = cache.get_mut(path.to_str().unwrap()) {
				let modify_time = to_seconds(path.metadata()?.modified()?);
				if *cached_time < modify_time {
					*cached_stats = internal::run_git()?;
					*cached_time = to_seconds(path.metadata()?.modified()?);
				}
				Ok(cached_stats.clone())
			} else {
				let curr_stats = internal::run_git()?;
				let curr_time = to_seconds(path.metadata()?.modified()?);
				cache.insert(path.clone().into_os_string().into_string().unwrap(), (curr_stats.clone(), curr_time));
				Ok(curr_stats)
			}
		} else {
			internal::run_git()
		}
	}
}

#[cfg(feature = "git-file-cache")]
impl<S> Drop for Git<S> {
	fn drop(&mut self) {
		if let Some(file_path) = self.file_path.take() {
			if let Some(cache) = self.cache.take() {
				#[cfg(feature = "git-file-cache")]
				std::fs::write(file_path, miniserde::json::to_string(&cache)).expect("failed writing git cache");
			}
		}
	}
}

impl GitStats {
	pub fn is_dirty(&self) -> bool {
		(self.untracked + self.conflicted + self.staged + self.non_staged) > 0
	}
}

fn find_git_dir() -> Option<path::PathBuf> {
	let mut git_dir = env::current_dir().unwrap();
	loop {
		git_dir.push(".git/");

		if git_dir.exists() {
			git_dir.pop();
			return Some(git_dir);
		}
		git_dir.pop();

		if !git_dir.pop() {
			return None;
		}
	}
}

impl<S: GitScheme> Module for Git<S> {
	fn append_segments(&mut self, segments: &mut Vec<Segment>) -> R<()> {
		let git_dir = match find_git_dir() {
			Some(dir) => dir,
			_ => return Ok(()),
		};

		let stats = self.get_git_data(git_dir)?;

		let (branch_fg, branch_bg) = if stats.is_dirty() {
			(S::GIT_REPO_DIRTY_FG, S::GIT_REPO_DIRTY_BG)
		} else {
			(S::GIT_REPO_CLEAN_FG, S::GIT_REPO_CLEAN_BG)
		};

		segments.push(Segment::simple(format!(" {} ", stats.branch_name), branch_fg, branch_bg));

		let mut add_elem = |count, symbol, fg, bg| {
			let quantity = |val: u32| -> String {
				if val > 1 {
					format!("{}", val)
				} else {
					String::new()
				}
			};

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

		Ok(())
	}
}
