#![crate_type = "lib"]
#![feature(globs,phase,macro_rules)]
//#![phase(syntax, link)] extern crate log;

extern crate strand;

pub mod drossel;
pub mod commands;