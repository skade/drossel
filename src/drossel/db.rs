use super::store::*;
use std::collections::dlist::DList;
use strand::mutable::Strand;
use strand::mutable::{Event,AsSendableEvent};
use strand::strand::Mutable;
use strand::errors::{Errors};
use std::collections::Deque;

use commands::ping;
use commands::get;
use commands::set;
use commands::command::*;

#[deriving(PartialEq,Show,Send)]
pub enum DBResult {
  Pong,
  Inserted(String),
  Removed(String, Option<Vec<u8>>)
}

pub struct DB {
  queue: Queue
}

impl DB {
  pub fn new() -> DB {
    DB { queue: Queue { state: DList::new() } }
  }

  pub fn execute(&mut self, effect: &Event<BinaryList,DBResult>) -> Result<DBResult, Errors> {
    self.queue.evolve(effect)
  }

  pub fn items(&self) -> uint {
    self.queue.state.len()
  }
}

impl Event<BinaryList, DBResult> for ping::Ping {
  fn precondition(&self, _: &BinaryList) -> Result<(), Errors> {
    Ok(())
  }

  fn action(&self, _: &mut BinaryList) -> Result<DBResult, Errors> {
    Ok(Pong)
  }

  fn postcondition(&self, _: &BinaryList) -> Result<(), Errors> {
    Ok(())
  }
}

impl Event<BinaryList, DBResult> for get::Get {
  fn precondition(&self, _: &BinaryList) -> Result<(), Errors> {
    Ok(())
  }

  fn action(&self, state: &mut BinaryList) -> Result<DBResult, Errors> {
    let res = state.pop_front();
    Ok(Removed(self.queue_name().clone(), res))
  }

  fn postcondition(&self, _: &BinaryList) -> Result<(), Errors> {
    Ok(())
  }
}

impl Event<BinaryList, DBResult> for set::Set {
  fn precondition(&self, _: &BinaryList) -> Result<(), Errors> {
    Ok(())
  }

  fn action(&self, state: &mut BinaryList) -> Result<DBResult, Errors> {
    state.push(self.payload().clone());
    Ok(Inserted(self.queue_name().clone()))
  }

  fn postcondition(&self, _: &BinaryList) -> Result<(), Errors> {
    Ok(())
  }
}

impl AsSendableEvent<BinaryList, DBResult> for Command {
  fn as_sendable_event(self) -> Box<Event<BinaryList, DBResult>+Send> {
    match self {
      Ping(p) => box p as Box<Event<BinaryList, DBResult>+Send>,
      Get(g) => box g as Box<Event<BinaryList, DBResult>+Send>,
      Set(s) => box s as Box<Event<BinaryList, DBResult>+Send>,
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  use strand::mutable::{AsSendableEvent};
  use commands::util::get_command;

  #[test]
  fn test_db_ping() {
    let command = get_command("PING".as_bytes().to_vec()).unwrap();
    let mut db = DB::new();
    let event = (*command).as_sendable_event();
    let res = db.execute(event);
    assert_eq!(Pong, res.unwrap())
  }

  #[test]
  fn test_db_set() {
    let command = get_command("SET test_queue 0 test_string".as_bytes().to_vec()).unwrap();
    let mut db = DB::new();
    let event = (*command).as_sendable_event();
    let res = db.execute(event);
    assert!(res.is_ok());
    assert_eq!(1, db.items())
  }

  #[test]
  fn test_db_get() {
    let set = get_command("SET test_queue 0 test_string".as_bytes().to_vec()).unwrap();
    let mut db = DB::new();
    let event1 = (*set).as_sendable_event();
    assert!(db.execute(event1).is_ok());
    let get = get_command("GET test_queue".as_bytes().to_vec()).unwrap();
    let event2 = (*get).as_sendable_event();
    assert!(db.execute(event2).is_ok());
    assert_eq!(0, db.items())
  }
}
