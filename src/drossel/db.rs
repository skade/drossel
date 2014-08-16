use super::store::*;
use std::collections::dlist::DList;
use super::super::*;
use strand::mutable::Strand;
use strand::mutable::Event;
use strand::strand::Mutable;
use strand::strand;
use strand::errors::{Errors};
use std::collections::Deque;

use drossel::events::{AsEvent};
use commands::ping;
use commands::get;
use commands::set;
use commands::command::*;

pub struct DB {
  queue: Queue
}

impl DB {
  pub fn new() -> DB {
    DB { queue: Queue { state: DList::new() } }
  }

  pub fn execute(&mut self, effect: &Event<BinaryList,&'static str>) -> Result<&'static str, Errors> {
    self.queue.evolve(effect)
  }

  pub fn items(&self) -> uint {
    self.queue.state.len()
  }
}

impl Event<BinaryList, &'static str> for ping::Ping {
  fn precondition(&self, _: &BinaryList) -> Result<(), Errors> {
    Ok(())
  }

  fn action(&self, state: &mut BinaryList) -> Result<&'static str, Errors> {
    Ok("PONG")
  }

  fn postcondition(&self, _: &BinaryList) -> Result<(), Errors> {
    Ok(())
  }
}

impl Event<BinaryList, &'static str> for get::Get {
  fn precondition(&self, _: &BinaryList) -> Result<(), Errors> {
    Ok(())
  }

  fn action(&self, state: &mut BinaryList) -> Result<&'static str, Errors> {
    state.pop_front();
    Ok("GET")
  }

  fn postcondition(&self, _: &BinaryList) -> Result<(), Errors> {
    Ok(())
  }
}

impl Event<BinaryList, &'static str> for set::Set {
  fn precondition(&self, _: &BinaryList) -> Result<(), Errors> {
    Ok(())
  }

  fn action(&self, state: &mut BinaryList) -> Result<&'static str, Errors> {
    state.push(self.payload().clone());
    Ok("SET")
  }

  fn postcondition(&self, _: &BinaryList) -> Result<(), Errors> {
    Ok(())
  }
}

impl AsEvent<BinaryList, &'static str> for Command {
  fn as_event<'a>(self, fun: |a: &Event<BinaryList, &'static str>| -> Result<&'static str, Errors>) -> Result<&'static str, Errors> {
    match self {
      Ping(p) => fun(&p as &Event<BinaryList, &'static str>),
      Get(g) => fun(&g as &Event<BinaryList, &'static str>),
      Set(s) => fun(&s as &Event<BinaryList, &'static str>),
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  use drossel::store::*;
  use std::collections::dlist::DList;
  use drossel::events::{AsEvent};
  use commands::util::get_command;
  use strand::mutable::Strand;
  use strand::mutable::Event;
  use strand::strand::Mutable;
  use strand::strand;
  use strand::errors::{Errors};

  #[test]
  fn test_db_ping() {
    let command = get_command("PING".as_bytes().to_vec()).unwrap();
    let mut db = box DB::new();
    let res = (*command).as_event(|event| {
      db.execute(event)
    });
    assert_eq!("PONG", res.unwrap())
  }

  #[test]
  fn test_db_set() {
    let command = get_command("SET test_queue 0 test_string".as_bytes().to_vec()).unwrap();
    let mut db = box DB::new();
    let res = (*command).as_event(|event| {
      db.execute(event)
    });
    assert_eq!(1, db.items())
  }

  #[test]
  fn test_db_get() {
    let set = get_command("SET test_queue 0 test_string".as_bytes().to_vec()).unwrap();
    let mut db = box DB::new();
    let res = (*set).as_event(|event| {
      db.execute(event)
    });
    let get = get_command("GET test_queue".as_bytes().to_vec()).unwrap();
    let res = (*get).as_event(|event| {
      db.execute(event)
    });
    assert_eq!(0, db.items())
  }
}