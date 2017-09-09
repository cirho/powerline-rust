use color::Color;
use part::Error;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use cpython::{Python, PyDict, ObjectProtocol, FromPyObject};

pub struct Theme {
    // TODO: use _something_ faster than HashMap
    colors: HashMap<Color, i32>,
}

impl Theme {

    pub fn get(&self, color: Color) -> i32 {
        self.colors[&color]
    }

    pub fn new_from_python() -> Result<Theme, Error> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let theme_file = read_file(&Theme::filename())?;
        let code = format!("{}{}", DEFAULT_COLOR_CLASS_PYTHON_CODE, theme_file);

        let locals = PyDict::new(py);
        py.run(&code, None, Some(&locals)).unwrap();

        let compiled = locals.get_item(py, "Color").unwrap();
        let mut colors = HashMap::new();

        macro_rules! add_property {
            ($prop:ident) => {
                colors.insert(Color::$prop, FromPyObject::extract(py, &compiled.getattr(py, stringify!($prop)).unwrap()).unwrap());
            }
        }
        add_property!(USERNAME_FG);
        add_property!(USERNAME_BG);
        add_property!(USERNAME_ROOT_BG);

        add_property!(HOSTNAME_FG);
        add_property!(HOSTNAME_BG);

        add_property!(HOME_BG);
        add_property!(HOME_FG);
        add_property!(PATH_BG);
        add_property!(PATH_FG);
        add_property!(CWD_FG);
        add_property!(SEPARATOR_FG);

        add_property!(READONLY_BG);
        add_property!(READONLY_FG);

        add_property!(SSH_BG);
        add_property!(SSH_FG);

        add_property!(REPO_CLEAN_BG);
        add_property!(REPO_CLEAN_FG);
        add_property!(REPO_DIRTY_BG);
        add_property!(REPO_DIRTY_FG);

        add_property!(JOBS_FG);
        add_property!(JOBS_BG);

        add_property!(CMD_PASSED_BG);
        add_property!(CMD_PASSED_FG);
        add_property!(CMD_FAILED_BG);
        add_property!(CMD_FAILED_FG);

        add_property!(SVN_CHANGES_BG);
        add_property!(SVN_CHANGES_FG);

        add_property!(GIT_AHEAD_BG);
        add_property!(GIT_AHEAD_FG);
        add_property!(GIT_BEHIND_BG);
        add_property!(GIT_BEHIND_FG);
        add_property!(GIT_STAGED_BG);
        add_property!(GIT_STAGED_FG);
        add_property!(GIT_NOTSTAGED_BG);
        add_property!(GIT_NOTSTAGED_FG);
        add_property!(GIT_UNTRACKED_BG);
        add_property!(GIT_UNTRACKED_FG);
        add_property!(GIT_CONFLICTED_BG);
        add_property!(GIT_CONFLICTED_FG);

        add_property!(VIRTUAL_ENV_BG);
        add_property!(VIRTUAL_ENV_FG);

        Ok(Theme { colors })
    }



    fn filename() -> String {
        format!("{}/{}", env::home_dir().unwrap().to_str().unwrap(), "/.config/powerline-rust/theme.py")
    }

}

fn read_file(path: &str) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
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
