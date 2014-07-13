#![crate_type = "lib"]
#![feature(globs,phase,macro_rules)]
//#![phase(syntax, link)] extern crate log;

extern crate strand;

use std::from_str::FromStr;

pub mod drossel;

macro_rules! command(
  ($com:ident, $str:expr, $len:expr) => ( // invoke it like `(input_5 SpecialE)`
    pub struct $com;

    impl FromStr for Box<$com> {
      fn from_str(s: &str) -> Option<Box<$com>> {
        if s == $str {
          Some(box $com)
        } else {
          None
        }
      }
    }
  );
)

macro_rules! cmd_from_string(
  ($com:ident, $var:expr) => (
    match from_str::<Box<$com>>($var) {
      Some(cmd) => { return Some(cmd as Box<Command>) },
      None => { None }
    };
  );
)

pub trait Command {
  fn execute(&self, args: &[&[u8]]) -> Vec<u8>;
  fn timeout(&self);
}

pub trait AcknowledgeableCommand {
  fn acknowledge(&self);
}

command!(Ping, "PING", 4)

impl Command for Ping {
  fn execute(&self, _args: &[&[u8]]) -> Vec<u8> {
    "PONG".as_bytes().to_owned()
  }
  fn timeout(&self) { }
}

command!(Get, "GET", 3)

impl Command for Get {
  fn execute(&self, args: &[&[u8]]) -> Vec<u8> {
    let subargs_str = args[0];
    let subargs : Vec<&[u8]> = subargs_str.split(|ch| ch == &('/' as u8)).collect();
    let queue_name = subargs.get(0);
    let command_args = subargs.tail();
    let result = format!("Command: GET queue: {}, args: {}", queue_name, command_args);
    result.as_bytes().to_owned()
  }
  fn timeout(&self) { }
}

command!(Set, "SET", 3)

impl Command for Set {
  fn execute(&self, args: &[&[u8]]) -> Vec<u8> {
    let queue_name = args[0];
    let expiration = args[1];
    let payload = args[2];
    let result = format!("Command: SET queue: {}, expiration: {}, payload: {}", queue_name, expiration, payload);
    result.as_bytes().to_owned()
  }
  fn timeout(&self) { }
}

impl FromStr for Box<Command> {
  fn from_str(s: &str) -> Option<Box<Command>> {
    let trimmed = s.trim();
    let mut command = cmd_from_string!(Ping, trimmed);
    command = command.or(cmd_from_string!(Get, trimmed));
    command = command.or(cmd_from_string!(Set, trimmed));
    command
  }
}
