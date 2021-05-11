use std::path::Path;
use std::process::Command;

use super::GitStats;

pub fn get_first_number(s: &str) -> u32 {
    s.chars().take_while(|x| x.is_digit(10)).flat_map(|x| x.to_digit(10)).fold(0, |acc, x| 10 * acc + x)
}

pub fn extract_ahead_behind(s: &str) -> (u32, u32) {
    let extract_number = |pos: usize, offset: usize| -> u32 {
        let s = s.get((pos + offset)..).unwrap();
        get_first_number(s)
    };
    let ahead = s.find("ahead").map(|pos| extract_number(pos, 6)).unwrap_or(0);
    let behind = s.find("behind").map(|pos| extract_number(pos, 7)).unwrap_or(0);
    (ahead, behind)
}

pub fn get_branch_name(s: &str) -> Option<&str> {
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

pub fn get_detached_branch_name() -> String {
    let child = Command::new("git").args(&["describe", "--tags", "--always"]).output().unwrap();

    if child.status.success() {
        let branch = std::str::from_utf8(&child.stdout).unwrap().split('\n').next().unwrap();
        format!("\u{2693}{}", branch)
    } else {
        String::from("Big Bang")
    }
}

pub fn run_git(_: &Path) -> GitStats {
    let output = Command::new("git").args(&["status", "--porcelain", "-b"]).output().unwrap().stdout;

    let mut lines = output.split(|x| *x == (b'\n'));
    let branch_line = std::str::from_utf8(lines.next().unwrap()).unwrap();

    let mut ahead = 0;
    let mut behind = 0;
    let mut non_staged = 0;
    let mut staged = 0;
    let mut conflicted = 0;
    let mut untracked = 0;

    let branch_name = {
        if let Some(branch_name) = get_branch_name(&branch_line) {
            if let Some(info) = branch_line.find('[').map(|pos| branch_line.get(pos..).unwrap()) {
                let (a, b) = extract_ahead_behind(info);
                ahead = a;
                behind = b;
            }
            String::from(branch_name)
        } else {
            get_detached_branch_name()
        }
    };
    let mut add_file = |entry: &str| {
        match entry {
            "??" => untracked += 1,
            "DD" | "AU" | "UD" | "UA" | "UU" | "DU" | "AA" => conflicted += 1,
            _ => {
                let mut chars = entry.chars();
                let a = chars.next().expect("invalid file status");
                let b = chars.next().expect("invalid file status");
                if b != ' ' {
                    non_staged += 1;
                }
                if a != ' ' {
                    staged += 1;
                }
            },
        };
    };
    for op in lines.flat_map(|line| line.get(..2)) {
        add_file(std::str::from_utf8(op).unwrap());
    }

    super::GitStats { untracked, ahead, behind, non_staged, staged, conflicted, branch_name }
}
