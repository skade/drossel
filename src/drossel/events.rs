use drossel::journal::*;
use strand::mutable::{Event,AsEvent};
use strand::errors::{Errors};
use drossel::types::*;
use commands::ping;
use commands::get;
use commands::set;
use commands::command::*;

impl Event<Journal, DBResult> for ping::Ping {
  fn precondition(&self, _: &Journal) -> Result<(), Errors> {
    Ok(())
  }

  fn action(&self, _: &mut Journal) -> Result<DBResult, Errors> {
    Ok(Pong)
  }

  fn postcondition(&self, _: &Journal) -> Result<(), Errors> {
    Ok(())
  }
}

impl Event<Journal, DBResult> for get::Get {
  fn precondition(&self, _: &Journal) -> Result<(), Errors> {
    Ok(())
  }

  fn action(&self, state: &mut Journal) -> Result<DBResult, Errors> {
    let res = state.pop(!self.open());
    Ok(Removed(self.queue_name().clone(), res))
  }

  fn postcondition(&self, _: &Journal) -> Result<(), Errors> {
    Ok(())
  }
}

impl Event<Journal, DBResult> for set::Set {
  fn precondition(&self, _: &Journal) -> Result<(), Errors> {
    Ok(())
  }

  fn action(&self, state: &mut Journal) -> Result<DBResult, Errors> {
    state.push(self.payload().clone().as_slice());
    Ok(Inserted(self.queue_name().clone()))
  }

  fn postcondition(&self, _: &Journal) -> Result<(), Errors> {
    Ok(())
  }
}

impl AsEvent<Journal, DBResult> for Command {
  fn as_event(self) -> Box<Event<Journal, DBResult>+Send> {
    match self {
      Ping(p) => box p as Box<Event<Journal, DBResult>+Send>,
      Get(g) => box g as Box<Event<Journal, DBResult>+Send>,
      Set(s) => box s as Box<Event<Journal, DBResult>+Send>,
    }
  }
}
