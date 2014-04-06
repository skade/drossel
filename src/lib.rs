#![crate_type = "lib"]
#![crate_id = "drossel#0.0.1"]
#![feature(globs,phase,macro_rules)]
#![phase(syntax, link)] extern crate log;

use std::from_str::FromStr;

macro_rules! command(
  ($com:ident, $str:expr, $len:expr, $execute:block, $timeout:block) => ( // invoke it like `(input_5 SpecialE)`
    pub struct $com;

    impl FromStr for ~$com {
      fn from_str(s: &str) -> Option<~$com> {
        if s.len() == $len && s.slice_to($len) == &$str {
          Some(~$com)
        } else {
          None
        }
      }
    }

    impl Command for $com {
      fn execute(&self, args: &[&[u8]]) -> ~[u8] $execute
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
  fn execute(&self, args: &[&[u8]]) -> ~[u8];
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

command!(Get, "GET", 3,
  {
    let subargs_str = args[0];
    let subargs: ~[&[u8]] = subargs_str.split(|ch| ch == &('/' as u8)).collect();
    let queue_name = subargs[0];
    let command_args = subargs.tail();
    let result = format!("Command: GET queue: {}, args: {}", queue_name, command_args);
    result.into_bytes()
  },
  {}
)

command!(Set, "SET", 3,
  {
    let subargs_str = args[0];
    let subargs: ~[&[u8]] = subargs_str.split(|ch| ch == &('/' as u8)).collect();
    let queue_name = subargs[0];
    let command_args = subargs.tail();
    let result = format!("Command: SET queue: {}, args: {}", queue_name, command_args);
    result.into_bytes()
  },
  {}
)

impl FromStr for ~Command {
  fn from_str(s: &str) -> Option<~Command> {
    let mut command = cmd_from_string!(Ping);
    command = command.or(cmd_from_string!(Hoge));
    command = command.or(cmd_from_string!(Get));
    command = command.or(cmd_from_string!(Set));
    command
  }
}