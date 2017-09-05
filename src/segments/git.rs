use ::color::Color;
use ::powerline::*;
use std::process::{Command, Stdio};
use std::str;

struct GitInfo {
    untracked: u32,
    conflicted: u32,
    non_staged: u32,
    ahead: u32,
    behind: u32,
    staged: u32,
}

impl GitInfo{
    fn new() -> GitInfo{
        GitInfo{ untracked: 0, conflicted: 0, non_staged: 0, staged: 0, ahead: 0, behind: 0}
    }

    fn is_dirty(&self) -> bool {
        (self.untracked + self.conflicted + self.staged + self.non_staged) > 0
    }

    fn add_file(&mut self, begin: &str) {
        match begin{
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
                let a = chars.next().unwrap();
                let b = chars.next().unwrap();
                if b != ' '{ self.non_staged += 1; }
                if a != ' '{ self.staged += 1; }
            },
        }
    }

}

fn get_detached_branch_name() -> String {
    let child = Command::new("git").args(&["describe", "--tags", "--always"]).output().expect("Failed to run git");
    if child.status.success() {
         return String::from("Big Bang")
    }
    let branch = str::from_utf8(&child.stdout).unwrap().split("\n").next().unwrap();
    format!("\u{2693}{} ",branch)
}

fn  quantity(val: u32) -> String{
    if val  > 1 { return format!("{}",val); }
    String::from("")
}
pub fn add_segment(powerline: &mut Powerline) {
    let output = Command::new("git").arg("status").arg("--porcelain").arg("-b").output().expect("Failed to run git");
    let data = str::from_utf8(&output.stdout).unwrap();
    if data == "" { return;}
    let mut git = GitInfo::new();
    let mut lines:Vec<&str> = data.split("\n").collect();
    lines.pop();

    let mut iter = lines.into_iter();
    let branch_line = iter.next().unwrap();

    iter.map(|x| x.get(..2).unwrap()).for_each(|x| git.add_file(x));

    let mut bg = Color::REPO_CLEAN_BG;
    let mut fg = Color::REPO_CLEAN_FG;
    if git.is_dirty(){
        bg = Color::REPO_DIRTY_BG;
        fg = Color::REPO_DIRTY_FG
    }
    powerline.add_segment(Segment::simple(branch_line.get(2..).unwrap(), fg, bg));
    if git.ahead > 0{
        let text = format!("{} \u{2B06} ", quantity(git.ahead));
        powerline.add_segment(Segment::simple(&text, Color::GIT_AHEAD_FG, Color::GIT_AHEAD_BG));
    }
    if git.behind > 0{
        let text = format!(" {}\u{2B07} ", quantity(git.behind));
        powerline.add_segment(Segment::simple(&text, Color::GIT_BEHIND_FG, Color::GIT_BEHIND_BG));
    }
    if git.staged > 0{
        let text = format!(" {}\u{2714} ", quantity(git.staged));
        powerline.add_segment(Segment::simple(&text, Color::GIT_STAGED_FG, Color::GIT_STAGED_BG));
    }
    if git.non_staged > 0{
        let text = format!(" {}\u{270E} ", quantity(git.non_staged));
        powerline.add_segment(Segment::simple(&text, Color::GIT_NOTSTAGED_FG, Color::GIT_NOTSTAGED_BG));
    }
    if git.untracked > 0{
        let text = format!(" {}\u{2753} ", quantity(git.untracked));
        powerline.add_segment(Segment::simple(&text, Color::GIT_UNTRACKED_FG, Color::GIT_UNTRACKED_BG));
    }
    if git.conflicted > 0{
        let text = format!(" {}\u{273C} ", quantity(git.conflicted));
        powerline.add_segment(Segment::simple(&text, Color::GIT_CONFLICTED_FG, Color::GIT_CONFLICTED_BG));
    }

}
