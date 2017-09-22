use color::Color;
use part::Error;
use std::env;
use std::fs::File;
use std::io::Read;
use cpython::{Python, PyObject, PyDict, ObjectProtocol, FromPyObject};
use std::{self, io, fs};
use bincode;

#[derive(Deserialize, Serialize, Clone)]
pub struct Theme {
    username_fg: u8,
    username_bg: u8,
    username_root_bg: u8,

    hostname_fg: u8,
    hostname_bg: u8,

    home_bg: u8,
    home_fg: u8,
    path_bg: u8,
    path_fg: u8,
    cwd_fg: u8,
    separator_fg: u8,

    readonly_bg: u8,
    readonly_fg: u8,

    ssh_bg: u8,
    ssh_fg: u8,

    repo_clean_bg: u8,
    repo_clean_fg: u8,
    repo_dirty_bg: u8,
    repo_dirty_fg: u8,

    jobs_fg: u8,
    jobs_bg: u8,

    cmd_passed_bg: u8,
    cmd_passed_fg: u8,
    cmd_failed_bg: u8,
    cmd_failed_fg: u8,

    svn_changes_bg: u8,
    svn_changes_fg: u8,

    git_ahead_bg: u8,
    git_ahead_fg: u8,
    git_behind_bg: u8,
    git_behind_fg: u8,
    git_staged_bg: u8,
    git_staged_fg: u8,
    git_notstaged_bg: u8,
    git_notstaged_fg: u8,
    git_untracked_bg: u8,
    git_untracked_fg: u8,
    git_conflicted_bg: u8,
    git_conflicted_fg: u8,

    virtual_env_bg: u8,
    virtual_env_fg: u8,
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
            Color::JOBS_FG => self.jobs_fg,
            Color::JOBS_BG => self.jobs_bg,
            Color::CMD_PASSED_BG => self.cmd_passed_bg,
            Color::CMD_PASSED_FG => self.cmd_passed_fg,
            Color::CMD_FAILED_BG => self.cmd_failed_bg,
            Color::CMD_FAILED_FG => self.cmd_failed_fg,
            Color::SVN_CHANGES_BG => self.svn_changes_bg,
            Color::SVN_CHANGES_FG => self.svn_changes_fg,
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
            Color::VIRTUAL_ENV_BG => self.virtual_env_bg,
            Color::VIRTUAL_ENV_FG => self.virtual_env_fg,
        }
    }

    pub fn new() -> Result<Theme, Error> {
        if Theme::is_cache_old()? {
            fs::remove_file(Theme::cache_filename())?;
        }

        Ok(if let Some(theme) = Theme::new_from_cache()? {
            theme
        } else if let Some(theme) = Theme::new_from_python()? {
            theme.update_cache()?;
            theme
        } else {
            DEFAULT_THEME.clone()
        })
    }

    fn update_cache(&self) -> Result<(), Error> {
        let mut file = fs::OpenOptions::new().create(true).write(true).truncate(true).open(&Theme::cache_filename())?;
        bincode::serialize_into(&mut file, &self, bincode::Infinite)?;
        Ok(())
    }

    fn is_cache_old() -> Result<bool, Error> {
        match fs::metadata(&Theme::cache_filename()) {
            Ok(cache_meta) => match fs::metadata(&Theme::theme_filename()) {
                Ok(theme_meta) => Ok(cache_meta.modified()? <= theme_meta.modified()?),
                Err(ref e) if e.kind() == io::ErrorKind::NotFound => Ok(true),
                Err(e) => Err(std::convert::From::from(e)),
            },
            Err(ref e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(std::convert::From::from(e)),
        }
    }

    fn new_from_cache() -> Result<Option<Theme>, Error> {
        match File::open(&Theme::cache_filename()) {
            Ok(mut file) => Ok(Some(bincode::deserialize_from(&mut file, bincode::Infinite)?)),
            Err(ref e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(std::convert::From::from(e)),
        }
    }

    fn new_from_python() -> Result<Option<Theme>, Error> {
        let theme_file = match File::open(&Theme::theme_filename()) {
            Ok(mut file) => {
                let mut s = String::new();
                file.read_to_string(&mut s)?;
                s
            },
            Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
                return Ok(None)
            },
            Err(e) => {
                return Err(std::convert::From::from(e))
            },
        };

        let gil = Python::acquire_gil();
        let py = gil.python();
        let compiled = Theme::python_compile(py, &theme_file)?;
        let get_prop = |prop: &str| -> u8 {
            FromPyObject::extract(py, &compiled.getattr(py, prop).unwrap()).unwrap()
        };
        Ok(Some(Theme {
            username_fg: get_prop("USERNAME_FG"),
            username_bg: get_prop("USERNAME_BG"),
            username_root_bg: get_prop("USERNAME_ROOT_BG"),
            hostname_fg: get_prop("HOSTNAME_FG"),
            hostname_bg: get_prop("HOSTNAME_BG"),
            home_bg: get_prop("HOME_BG"),
            home_fg: get_prop("HOME_FG"),
            path_bg: get_prop("PATH_BG"),
            path_fg: get_prop("PATH_FG"),
            cwd_fg: get_prop("CWD_FG"),
            separator_fg: get_prop("SEPARATOR_FG"),
            readonly_bg: get_prop("READONLY_BG"),
            readonly_fg: get_prop("READONLY_FG"),
            ssh_bg: get_prop("SSH_BG"),
            ssh_fg: get_prop("SSH_FG"),
            repo_clean_bg: get_prop("REPO_CLEAN_BG"),
            repo_clean_fg: get_prop("REPO_CLEAN_FG"),
            repo_dirty_bg: get_prop("REPO_DIRTY_BG"),
            repo_dirty_fg: get_prop("REPO_DIRTY_FG"),
            jobs_fg: get_prop("JOBS_FG"),
            jobs_bg: get_prop("JOBS_BG"),
            cmd_passed_bg: get_prop("CMD_PASSED_BG"),
            cmd_passed_fg: get_prop("CMD_PASSED_FG"),
            cmd_failed_bg: get_prop("CMD_FAILED_BG"),
            cmd_failed_fg: get_prop("CMD_FAILED_FG"),
            svn_changes_bg: get_prop("SVN_CHANGES_BG"),
            svn_changes_fg: get_prop("SVN_CHANGES_FG"),
            git_ahead_bg: get_prop("GIT_AHEAD_BG"),
            git_ahead_fg: get_prop("GIT_AHEAD_FG"),
            git_behind_bg: get_prop("GIT_BEHIND_BG"),
            git_behind_fg: get_prop("GIT_BEHIND_FG"),
            git_staged_bg: get_prop("GIT_STAGED_BG"),
            git_staged_fg: get_prop("GIT_STAGED_FG"),
            git_notstaged_bg: get_prop("GIT_NOTSTAGED_BG"),
            git_notstaged_fg: get_prop("GIT_NOTSTAGED_FG"),
            git_untracked_bg: get_prop("GIT_UNTRACKED_BG"),
            git_untracked_fg: get_prop("GIT_UNTRACKED_FG"),
            git_conflicted_bg: get_prop("GIT_CONFLICTED_BG"),
            git_conflicted_fg: get_prop("GIT_CONFLICTED_FG"),
            virtual_env_bg: get_prop("VIRTUAL_ENV_BG"),
            virtual_env_fg: get_prop("VIRTUAL_ENV_FG"),
        }))
    }

    fn python_compile(py: Python, theme_file: &str) -> Result<PyObject, Error> {
        let code = format!("{}{}", DEFAULT_COLOR_CLASS_PYTHON_CODE, theme_file);
        let locals = PyDict::new(py);
        py.run(&code, None, Some(&locals)).unwrap();
        let compiled = locals.get_item(py, "Color").unwrap();
        Ok(compiled)
    }


    fn theme_filename() -> String {
        format!("{}/{}", env::home_dir().unwrap().to_str().unwrap(), "/.config/powerline-rust/theme.py")
    }
    fn cache_filename() -> String {
        format!("{}/{}", env::home_dir().unwrap().to_str().unwrap(), "/.config/powerline-rust/.theme_cache")
    }

}

// https://github.com/banga/powerline-shell/blob/master/themes/default.py
static DEFAULT_COLOR_CLASS_PYTHON_CODE: &'static str = r#"

class DefaultColor:
    """
    This class should have the default colors for every segment.
    Please test every new segment with this theme first.
    """
    # RESET is not a real color code. It is used as in indicator
    # within the code that any foreground / background color should
    # be cleared
    RESET = -1

    USERNAME_FG = 250
    USERNAME_BG = 240
    USERNAME_ROOT_BG = 124

    HOSTNAME_FG = 250
    HOSTNAME_BG = 238

    HOME_SPECIAL_DISPLAY = True
    HOME_BG = 31  # blueish
    HOME_FG = 15  # white
    PATH_BG = 237  # dark grey
    PATH_FG = 250  # light grey
    CWD_FG = 254  # nearly-white grey
    SEPARATOR_FG = 244

    READONLY_BG = 124
    READONLY_FG = 254

    SSH_BG = 166 # medium orange
    SSH_FG = 254

    REPO_CLEAN_BG = 148  # a light green color
    REPO_CLEAN_FG = 0  # black
    REPO_DIRTY_BG = 161  # pink/red
    REPO_DIRTY_FG = 15  # white

    JOBS_FG = 39
    JOBS_BG = 238

    CMD_PASSED_BG = 236
    CMD_PASSED_FG = 15
    CMD_FAILED_BG = 161
    CMD_FAILED_FG = 15

    SVN_CHANGES_BG = 148
    SVN_CHANGES_FG = 22  # dark green

    GIT_AHEAD_BG = 240
    GIT_AHEAD_FG = 250
    GIT_BEHIND_BG = 240
    GIT_BEHIND_FG = 250
    GIT_STAGED_BG = 22
    GIT_STAGED_FG = 15
    GIT_NOTSTAGED_BG = 130
    GIT_NOTSTAGED_FG = 15
    GIT_UNTRACKED_BG = 52
    GIT_UNTRACKED_FG = 15
    GIT_CONFLICTED_BG = 9
    GIT_CONFLICTED_FG = 15

    VIRTUAL_ENV_BG = 35  # a mid-tone green
    VIRTUAL_ENV_FG = 00

"#;

static DEFAULT_THEME: Theme = Theme {
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

    jobs_fg: 39,
    jobs_bg: 238,

    cmd_passed_bg: 236,
    cmd_passed_fg: 15,
    cmd_failed_bg: 161,
    cmd_failed_fg: 15,

    svn_changes_bg: 148,
    svn_changes_fg: 22,

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

    virtual_env_bg: 35,
    virtual_env_fg: 00,
};
