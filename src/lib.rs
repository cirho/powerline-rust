#![feature(libc)]
extern crate libc;
extern crate users;

pub mod powerline;
pub mod segments;
pub mod part;
pub mod color;

pub use powerline::*;
