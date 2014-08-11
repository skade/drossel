#![crate_type = "lib"]
#![feature(globs,phase,macro_rules)]
//#![phase(syntax, link)] extern crate log;

extern crate strand;

use std::from_str::FromStr;
use std::str::from_utf8;

pub mod drossel;

pub trait Command {
  fn execute(&self) -> Vec<u8>;
  fn timeout(&self);
}

pub trait AcknowledgeableCommand {
  fn acknowledge(&self);
}

pub struct Ping;

impl Command for Ping {
  fn execute(&self) -> Vec<u8> {
    "PONG".as_bytes().to_vec()
  }
  fn timeout(&self) { }
}

pub struct Get {
  queue_name: String
}

impl Command for Get {
  fn execute(&self) -> Vec<u8> {
    let result = format!("Command: GET queue: {}", self.queue_name);
    result.as_bytes().to_vec()
  }
  fn timeout(&self) { }
}

pub struct Set{
  queue_name: String,
  expiration: String,
  payload: Vec<u8>
}

impl Command for Set {
  fn execute(&self) -> Vec<u8> {
    let result = format!("Command: SET queue: {}, expiration: {}, payload: {}", self.queue_name, self.expiration, self.payload);
    result.as_bytes().to_vec()
  }
  fn timeout(&self) { }
}

pub fn get_command(message: Vec<u8>) -> Option<Box<Command>> {
  let split_input: Vec<&[u8]> = message.as_slice().split(|ch| ch == &(' ' as u8) || ch == &('\n' as u8)).collect();
  match from_utf8(split_input[0]).unwrap().trim() {
    "PING" => { Some(box Ping as Box<Command>) },
    "GET"  => {
      let get = Get { queue_name: from_utf8(split_input[1]).unwrap().to_string() };
      Some(box get as Box<Command>)
    },
    "SET"  => {
      let set = Set {
                      queue_name: from_utf8(split_input[1]).unwrap().to_string(),
                      expiration: from_utf8(split_input[2]).unwrap().to_string(),
                      payload: split_input[3].to_vec()
                    };
      Some(box set as Box<Command>)
    },
    _ => { None }
  }
}