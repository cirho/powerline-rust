use crate::{modules::*, terminal::Color};

#[derive(Copy, Clone)]
pub struct SimpleTheme;

impl CmdScheme for SimpleTheme {
	const CMD_FAILED_BG: Color = Color(161);
	const CMD_FAILED_FG: Color = Color(15);
	const CMD_PASSED_BG: Color = Color(236);
	const CMD_PASSED_FG: Color = Color(15);
}

impl CwdScheme for SimpleTheme {
	const CWD_FG: Color = Color(254);
	const HOME_BG: Color = Color(31);
	const HOME_FG: Color = Color(15);
	const PATH_BG: Color = Color(237);
	const PATH_FG: Color = Color(250);
	const SEPARATOR_FG: Color = Color(244);
}

impl NewLineScheme for SimpleTheme {
	const NEWLINE_BG: Color = Color(0);
	const NEWLINE_FG: Color = Color(0);
	const NEWLINE_ROOT_BG: Color = Color(0);
}

impl UserScheme for SimpleTheme {
	const USERNAME_BG: Color = Color(240);
	const USERNAME_FG: Color = Color(250);
	const USERNAME_ROOT_BG: Color = Color(124);
}

impl HostScheme for SimpleTheme {
	const HOSTNAME_BG: Color = Color(238);
	const HOSTNAME_FG: Color = Color(250);
}

impl ReadOnlyScheme for SimpleTheme {
	const READONLY_BG: Color = Color(124);
	const READONLY_FG: Color = Color(254);
}

impl GitScheme for SimpleTheme {
	const GIT_AHEAD_BG: Color = Color(240);
	const GIT_AHEAD_FG: Color = Color(250);
	const GIT_BEHIND_BG: Color = Color(240);
	const GIT_BEHIND_FG: Color = Color(250);
	const GIT_CONFLICTED_BG: Color = Color(9);
	const GIT_CONFLICTED_FG: Color = Color(15);
	const GIT_NOTSTAGED_BG: Color = Color(130);
	const GIT_NOTSTAGED_FG: Color = Color(15);
	const GIT_REPO_CLEAN_BG: Color = Color(148);
	const GIT_REPO_CLEAN_FG: Color = Color(0);
	const GIT_REPO_DIRTY_BG: Color = Color(161);
	const GIT_REPO_DIRTY_FG: Color = Color(15);
	const GIT_STAGED_BG: Color = Color(22);
	const GIT_STAGED_FG: Color = Color(15);
	const GIT_UNTRACKED_BG: Color = Color(52);
	const GIT_UNTRACKED_FG: Color = Color(15);
}
