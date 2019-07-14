use crate::color::Color;

#[derive(Copy, Clone)]
pub struct Theme {
	pub username_fg: u8,
	pub username_bg: u8,
	pub username_root_bg: u8,

	pub hostname_fg: u8,
	pub hostname_bg: u8,

	pub home_bg: u8,
	pub home_fg: u8,
	pub path_bg: u8,
	pub path_fg: u8,
	pub cwd_fg: u8,
	pub separator_fg: u8,

	pub readonly_bg: u8,
	pub readonly_fg: u8,

	pub ssh_bg: u8,
	pub ssh_fg: u8,

	pub repo_clean_bg: u8,
	pub repo_clean_fg: u8,
	pub repo_dirty_bg: u8,
	pub repo_dirty_fg: u8,

	pub cmd_passed_bg: u8,
	pub cmd_passed_fg: u8,
	pub cmd_failed_bg: u8,
	pub cmd_failed_fg: u8,

	pub git_ahead_bg: u8,
	pub git_ahead_fg: u8,
	pub git_behind_bg: u8,
	pub git_behind_fg: u8,
	pub git_staged_bg: u8,
	pub git_staged_fg: u8,
	pub git_notstaged_bg: u8,
	pub git_notstaged_fg: u8,
	pub git_untracked_bg: u8,
	pub git_untracked_fg: u8,
	pub git_conflicted_bg: u8,
	pub git_conflicted_fg: u8,
}

impl Theme {
	pub fn get(&self, color: Color) -> u8 {
		match color {
			Color::USERNAME_FG => self.username_fg,
			Color::USERNAME_BG => self.username_bg,
			Color::USERNAME_ROOT_BG => self.username_root_bg,
			Color::HOSTNAME_FG => self.hostname_fg,
			Color::HOSTNAME_BG => self.hostname_bg,
			Color::HOME_BG => self.home_bg,
			Color::HOME_FG => self.home_fg,
			Color::PATH_BG => self.path_bg,
			Color::PATH_FG => self.path_fg,
			Color::CWD_FG => self.cwd_fg,
			Color::SEPARATOR_FG => self.separator_fg,
			Color::READONLY_BG => self.readonly_bg,
			Color::READONLY_FG => self.readonly_fg,
			Color::SSH_BG => self.ssh_bg,
			Color::SSH_FG => self.ssh_fg,
			Color::REPO_CLEAN_BG => self.repo_clean_bg,
			Color::REPO_CLEAN_FG => self.repo_clean_fg,
			Color::REPO_DIRTY_BG => self.repo_dirty_bg,
			Color::REPO_DIRTY_FG => self.repo_dirty_fg,
			Color::CMD_PASSED_BG => self.cmd_passed_bg,
			Color::CMD_PASSED_FG => self.cmd_passed_fg,
			Color::CMD_FAILED_BG => self.cmd_failed_bg,
			Color::CMD_FAILED_FG => self.cmd_failed_fg,
			Color::GIT_AHEAD_BG => self.git_ahead_bg,
			Color::GIT_AHEAD_FG => self.git_ahead_fg,
			Color::GIT_BEHIND_BG => self.git_behind_bg,
			Color::GIT_BEHIND_FG => self.git_behind_fg,
			Color::GIT_STAGED_BG => self.git_staged_bg,
			Color::GIT_STAGED_FG => self.git_staged_fg,
			Color::GIT_NOTSTAGED_BG => self.git_notstaged_bg,
			Color::GIT_NOTSTAGED_FG => self.git_notstaged_fg,
			Color::GIT_UNTRACKED_BG => self.git_untracked_bg,
			Color::GIT_UNTRACKED_FG => self.git_untracked_fg,
			Color::GIT_CONFLICTED_BG => self.git_conflicted_bg,
			Color::GIT_CONFLICTED_FG => self.git_conflicted_fg,
		}
	}
}

pub static DEFAULT_THEME: Theme = Theme {
	username_fg: 250,
	username_bg: 240,
	username_root_bg: 124,

	hostname_fg: 250,
	hostname_bg: 238,

	home_bg: 31,
	home_fg: 15,
	path_bg: 237,
	path_fg: 250,
	cwd_fg: 254,
	separator_fg: 244,

	readonly_bg: 124,
	readonly_fg: 254,

	ssh_bg: 166,
	ssh_fg: 254,

	repo_clean_bg: 148,
	repo_clean_fg: 0,
	repo_dirty_bg: 161,
	repo_dirty_fg: 15,

	cmd_passed_bg: 236,
	cmd_passed_fg: 15,
	cmd_failed_bg: 161,
	cmd_failed_fg: 15,

	git_ahead_bg: 240,
	git_ahead_fg: 250,
	git_behind_bg: 240,
	git_behind_fg: 250,
	git_staged_bg: 22,
	git_staged_fg: 15,
	git_notstaged_bg: 130,
	git_notstaged_fg: 15,
	git_untracked_bg: 52,
	git_untracked_fg: 15,
	git_conflicted_bg: 9,
	git_conflicted_fg: 15,
};
