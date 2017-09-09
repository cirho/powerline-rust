#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(non_camel_case_types)]

#[derive(Copy, Hash, PartialEq, Eq)]
pub enum Color {
    USERNAME_FG,
    USERNAME_BG,
    USERNAME_ROOT_BG,

    HOSTNAME_FG,
    HOSTNAME_BG,

    HOME_BG,
    HOME_FG,
    PATH_BG,
    PATH_FG,
    CWD_FG,
    SEPARATOR_FG,

    READONLY_BG,
    READONLY_FG,

    SSH_BG,
    SSH_FG,

    REPO_CLEAN_BG,
    REPO_CLEAN_FG,
    REPO_DIRTY_BG,
    REPO_DIRTY_FG,

    JOBS_FG,
    JOBS_BG,

    CMD_PASSED_BG,
    CMD_PASSED_FG,
    CMD_FAILED_BG,
    CMD_FAILED_FG,

    SVN_CHANGES_BG,
    SVN_CHANGES_FG,

    GIT_AHEAD_BG,
    GIT_AHEAD_FG,
    GIT_BEHIND_BG,
    GIT_BEHIND_FG,
    GIT_STAGED_BG,
    GIT_STAGED_FG,
    GIT_NOTSTAGED_BG,
    GIT_NOTSTAGED_FG,
    GIT_UNTRACKED_BG,
    GIT_UNTRACKED_FG,
    GIT_CONFLICTED_BG,
    GIT_CONFLICTED_FG,

    VIRTUAL_ENV_BG,
    VIRTUAL_ENV_FG,
    Custom(i32),
}
impl Clone for Color{
    fn clone(&self) -> Color { *self }
}