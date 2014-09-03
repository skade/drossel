use super::store::*;
use strand::mutable::{Event,AsEvent};
use strand::errors::{Errors};
use drossel::types::*;
use commands::ping;
use commands::get;
use commands::set;
use commands::command::*;
use std::collections::Deque;


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

impl AsEvent<BinaryList, DBResult> for Command {
  fn as_event(self) -> Box<Event<BinaryList, DBResult>+Send> {
    match self {
      Ping(p) => box p as Box<Event<BinaryList, DBResult>+Send>,
      Get(g) => box g as Box<Event<BinaryList, DBResult>+Send>,
      Set(s) => box s as Box<Event<BinaryList, DBResult>+Send>,
    }
  }
}
