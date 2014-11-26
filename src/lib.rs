#![crate_type = "lib"]
#![feature(globs,phase,macro_rules)]
#![feature(if_let)]
//#![phase(syntax, link)] extern crate log;

extern crate db_key;
extern crate strand;
extern crate leveldb;
extern crate drossel_journal;

pub mod drossel;
pub mod commands;
