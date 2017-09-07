#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(non_camel_case_types)]

#[derive(Copy)]
pub enum Color {
    HOME_BG,
    HOME_FG,
    PATH_BG,
    PATH_FG,
    SEPARATOR_FG,
    CMD_PASSED_BG,
    CMD_PASSED_FG,
    CMD_FAILED_BG,
    CMD_FAILED_FG,
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
    REPO_CLEAN_BG,
    REPO_CLEAN_FG,
    REPO_DIRTY_BG,
    REPO_DIRTY_FG,
    USERNAME_FG,
    USERNAME_BG,
    HOSTNAME_FG,
    HOSTNAME_BG,

    Custom(i32),
}
impl Clone for Color{
    fn clone(&self) -> Color { *self }
}
impl Color {
    fn code(&self) -> i32 {
        match self{
            &Color::HOME_BG => 31,
            &Color::HOME_FG => 15,
            &Color::PATH_BG => 237,
            &Color::PATH_FG => 250,
            &Color::SEPARATOR_FG => 244,
            &Color::CMD_PASSED_BG => 236,
            &Color::CMD_PASSED_FG => 15,
            &Color::CMD_FAILED_BG => 161,
            &Color::CMD_FAILED_FG => 15,
            &Color::GIT_AHEAD_BG => 240,
            &Color::GIT_AHEAD_FG => 250,
            &Color::GIT_BEHIND_BG => 240,
            &Color::GIT_BEHIND_FG => 250,
            &Color::GIT_STAGED_BG => 22,
            &Color::GIT_STAGED_FG => 15,
            &Color::GIT_NOTSTAGED_BG => 130,
            &Color::GIT_NOTSTAGED_FG => 15,
            &Color::GIT_UNTRACKED_BG => 52,
            &Color::GIT_UNTRACKED_FG => 15,
            &Color::GIT_CONFLICTED_BG => 9,
            &Color::GIT_CONFLICTED_FG => 15,
            &Color::REPO_DIRTY_FG => 15,
            &Color::REPO_DIRTY_BG => 161,
            &Color::REPO_CLEAN_FG => 0,
            &Color::REPO_CLEAN_BG => 148,
            &Color::USERNAME_FG => 250,
            &Color::USERNAME_BG => 240,
            &Color::HOSTNAME_FG => 250,
            &Color::HOSTNAME_BG => 238,
            &Color::Custom(col) => col,
        }
    }

    pub fn bg_str(&self) -> String { format!("\\[\\e[48;5;{}m\\]", self.code()) }
    pub fn fg_str(&self) -> String { format!("\\[\\e[38;5;{}m\\]", self.code()) }
    pub fn reset() -> String { String::from("\\[\\e[0m\\]") }
}
