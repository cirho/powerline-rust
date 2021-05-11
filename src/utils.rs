use std::env;

pub fn is_remote_shell() -> bool {
    env::var_os("SSH_CLIENT").is_some() || env::var_os("SSH_TTY").is_some() || env::var_os("SSH_CONNECTION").is_some()
}
