#![feature(libc)]
extern crate libc;
extern crate users;

pub mod color;
pub mod part;
pub mod powerline;
pub mod segments;
pub mod theme;

pub use powerline::*;
