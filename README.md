# powerline-rust
[![Build Status](https://travis-ci.org/Xeoeen/powerline-rust.svg?branch=lib)](https://travis-ci.org/Xeoeen/powerline-rust)

powerline-rust is an altervative to [powerline-shell](https://github.com/b-ryan/powerline-shell). It's heavily inspired by it, but focuses on **minimalizing time of execution**.

Nobody wants to see latency between pressing enter in favourite shell and seeing prompt. This is main aim of this crate and that's why some features of other alternatives like dynamic segments choosing and theming via **commandline arguments** is **not possible here**.

Although, similar results **can be archived** by **customization**.

There is a demand to recompile every time while customizing, but you change your prompt only once upon a time. I think performance beneficence is worth it.

## Advantages 
- blazing fast (less than 0.010s)
- only necessary dependencies  ([users](https://crates.io/crates/users) crate)
- runs git backend only when needed (huge time improvements in directories not in git tree)
## Simple installation
```bash
git clone https://github.com/Xeoeen/powerline-rust
cd powerline-rust
cargo +nightly install
```
## Setting up shell
#### Make sure you have executable in `$PATH`
### Bash
```bash
function _update_ps1() {
    PS1="$(powerline $?)"
}

if [ "$TERM" != "linux" ]; then
    PROMPT_COMMAND="_update_ps1; $PROMPT_COMMAND"
fi
```

## Custom shell prompt
Simply create new rust program that fulfils your requirements.
```rust
extern crate powerline;

use powerline::{part::*, segments::*};

fn main() {
	let mut prompt = powerline::Powerline::new(powerline::theme::DEFAULT_THEME);
	// adjustable cwd segment cropping in case of deep path
	prompt.add_segments(cwd::Cwd::new("~", 45, 4).get_segments().expect("Failed seg: Cwd"));
	prompt.add_segments(git::GitInfo::new().get_segments().expect("Failed seg: Git"));
	// custom symblos everywhere (utf-8 support)
	prompt.add_segments(readonly::ReadOnly::new("").get_segments().expect("Failed seg: ReadOnly"));
	prompt.add_segments(cmd::Cmd::new("$", "#").get_segments().expect("Failed seg: Cmd"));
	println!("{}", prompt);
}

```
## Tips and trics
### Strip executable
Remove unnecessary symbols from file to greatly reduce size of it.
Theoretically it can reduce time of execution.
```bash
cd ~/.cargo/bin/
strip powerline
```
### Say goodbye to jemalloc and welcome system allocator
Again it can make everything faster.
```rust
// add at the top of source code
#[global_allocator]
static GLOBAL: std::alloc::System = std::alloc::System;
```
### Use LTO and other tricks
Same matter.
```rust
// Cargo.toml
[profile.release]
lto = true
panic = 'abort'
```

### Custom theme
Nothing more to add.
```rust
extern crate powerline;

use powerline::{part::*, segments::*};

fn custom_theme() -> powerline::theme::Theme {
	powerline::theme::Theme {
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
	}
}

fn main() {
	let mut prompt = powerline::Powerline::new(custom_theme());
...
```



