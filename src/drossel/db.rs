use super::store::*;
use std::collections::dlist::DList;
use strand::mutable::Strand;
use strand::mutable::{Event};
use strand::strand::Mutable;
use strand::errors::{Errors};
use drossel::types::DBResult;

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

#[cfg(test)]
mod tests {
  use drossel::db::{DB};
  use drossel::types::*;
  use strand::mutable::{AsEvent};
  use commands::util::get_command;

  #[test]
  fn test_db_ping() {
    let command = get_command("PING".as_bytes().to_vec()).unwrap();
    let mut db = DB::new();
    let event = (*command).as_event();
    let res = db.execute(event);
    assert_eq!(Pong, res.unwrap())
  }

  #[test]
  fn test_db_set() {
    let command = get_command("SET test_queue test_string".as_bytes().to_vec()).unwrap();
    let mut db = DB::new();
    let event = (*command).as_event();
    let res = db.execute(event);
    assert!(res.is_ok());
    assert_eq!(1, db.items())
  }

  #[test]
  fn test_db_get() {
    let set = get_command("SET test_queue 0 test_string".as_bytes().to_vec()).unwrap();
    let mut db = DB::new();
    let event1 = (*set).as_event();
    assert!(db.execute(event1).is_ok());
    let get = get_command("GET test_queue".as_bytes().to_vec()).unwrap();
    let event2 = (*get).as_event();
    assert!(db.execute(event2).is_ok());
    assert_eq!(0, db.items())
  }
}
