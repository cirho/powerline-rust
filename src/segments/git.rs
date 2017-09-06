use ::color::Color;
use ::powerline::*;
use ::part::*;
use std::process::Command;
use std::str;
use regex::Regex;

pub struct GitInfo {
    untracked: u32,
    conflicted: u32,
    non_staged: u32,
    pub ahead: u32,
    pub behind: u32,
    staged: u32,
}

impl GitInfo{
    pub fn new() -> GitInfo{
        GitInfo{ untracked: 0, conflicted: 0, non_staged: 0, staged: 0, ahead: 0, behind: 0}
    }

    fn is_dirty(&self) -> bool {
        (self.untracked + self.conflicted + self.staged + self.non_staged) > 0
    }

    fn add_file(&mut self, begin: &str) -> Result<(), Error> {
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
                let a = chars.next().ok_or(Error::from_str("Invalid file status"))?;
                let b = chars.next().ok_or(Error::from_str("Invalid file status"))?;
                if b != ' '{ self.non_staged += 1; }
                if a != ' '{ self.staged += 1; }
            },
        };
        Ok(())
    }

}

fn get_detached_branch_name() -> Result<String, Error> {
    let child = Command::new("git").args(&["describe", "--tags", "--always"]).output().map_err(|e| Error::wrap(e, "Failed to run git"))?;
    if child.status.success() {
         return Ok(String::from("Big Bang"))
    }
    let branch = str::from_utf8(&child.stdout)?.split("\n").next().ok_or(Error::from_str("Empty git output"))?;
    Ok(format!("\u{2693}{}",branch))
}

fn  quantity(val: u32) -> String{
    if val  > 1 { return format!("{}",val); }
    String::from("")
}

impl Part for GitInfo {
fn segments(self) -> Result<Vec<Segment>, Error> {
    let output = Command::new("git").args(&["status", "--porcelain", "-b"]).output().map_err(|e| Error::wrap(e, "Failed to run git"))?;
    let data = str::from_utf8(&output.stdout)?;
    if data == "" { return Ok(Vec::new());}
    let mut git = GitInfo::new();
    let mut lines:Vec<&str> = data.split("\n").collect();
    lines.pop();

    let mut iter = lines.into_iter();
    let branch_line = iter.next().ok_or(Error::from_str("Empty git output"))?;
    let re = Regex::new(r"^## (?P<local>[^\.]+)?")?;

    let branch: String = {
        if let Some(caps) = re.captures(branch_line) {
            caps["local"].to_owned()
        }
        else {
            get_detached_branch_name()?
        }
    };
    for x in iter {
        let file = x.get(..2).ok_or(Error::from_str("Invalid file status line"))?;
        git.add_file(file)?;
    }

    let mut bg = Color::REPO_CLEAN_BG;
    let mut fg = Color::REPO_CLEAN_FG;
    if git.is_dirty(){
        bg = Color::REPO_DIRTY_BG;
        fg = Color::REPO_DIRTY_FG
    }
    let mut results = Vec::new();
    results.push(Segment::simple(&format!(" {} ",branch), fg, bg));
    //Maybe some funny macro would be better
    macro_rules! add_elem {
    	($count: expr, $fmtstr: expr, $fg:expr, $bg: expr) => {
    		if $count > 0 {
    			let text = format!($fmtstr, quantity($count));
                       results.push(Segment::simple(&text, $fg, $bg));
    		}
    	}
    }
    add_elem!(git.ahead, "{} \u{2B06} ", Color::GIT_AHEAD_FG, Color::GIT_AHEAD_BG);
    add_elem!(git.behind, "{} \u{2B07} ", Color::GIT_BEHIND_FG, Color::GIT_BEHIND_BG);
    add_elem!(git.staged, "{} \u{2714} ", Color::GIT_STAGED_FG, Color::GIT_STAGED_BG);
    add_elem!(git.non_staged, " {}\u{270E} ", Color::GIT_NOTSTAGED_FG, Color::GIT_NOTSTAGED_BG);
    add_elem!(git.untracked, " {}\u{2753} ", Color::GIT_UNTRACKED_FG, Color::GIT_UNTRACKED_BG);
    add_elem!(git.conflicted, " {}\u{273C} ", Color::GIT_CONFLICTED_FG, Color::GIT_CONFLICTED_BG);
    Ok(results)
}
}
