#![crate_type = "lib"]
#![crate_id = "drossel#0.0.1"]
#![feature(globs,phase,macro_rules)]
#![phase(syntax, link)] extern crate log;

use std::from_str::FromStr;
use std::str::eq_slice;

macro_rules! command(
  ($com:ident, $str:expr, $len:expr, $execute:block, $timeout:block) => ( // invoke it like `(input_5 SpecialE)`
    pub struct $com;

    impl FromStr for ~$com {
      fn from_str(s: &str) -> Option<~$com> {
        if s.slice_to($len) == &$str {
          Some(~$com)
        } else {
          None
        }
      }
    }

    impl Command for $com {
      fn execute(&self) -> ~[u8] $execute
      fn timeout(&self) $timeout
    }
  );
)

macro_rules! cmd_from_string(
  ($com:ident) => (
    match from_str::<~$com>(s) {
      Some(cmd) => { Some(cmd as ~Command) },
      None => { None }
    };
  );
)

pub trait Command {
  fn execute(&self) -> ~[u8];
  fn timeout(&self);
}

pub trait AcknowledgeableCommand {
  fn acknowledge(&self);
}

command!(Ping, "PING", 4,
  {
    (~"PONG").into_bytes()
  },
  {}
)

command!(Hoge, "HOGE", 4,
  {
    (~"FUGE").into_bytes()
  },
  {}
)

impl FromStr for ~Command {
  fn from_str(s: &str) -> Option<~Command> {
    let mut command = cmd_from_string!(Ping);
    command = command.or(cmd_from_string!(Hoge));
    command
  }
}