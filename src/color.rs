#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(non_camel_case_types)]

#[derive(Copy)]
pub enum Color {
    USERNAME_FG,
    USERNAME_BG,
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
    CMD_PASSED_BG,
    CMD_PASSED_FG,
    CMD_FAILED_BG,
    CMD_FAILED_FG,
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
            &Color::Custom(col) => col,
            _ => panic!("Non matching value to color"),
        }
    }

    pub fn bg_str(&self) -> String { format!("\\[\\e[48;5;{}m\\]", self.code()) }
    pub fn fg_str(&self) -> String { format!("\\[\\e[38;5;{}m\\]", self.code()) }
    pub fn reset() -> String { String::from("\\[\\e[0m\\]") }
}
