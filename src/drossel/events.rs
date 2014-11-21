use drossel::journal::*;
use strand::mutable::{Event,AsEvent};
use strand::errors::{Errors};
use drossel::types::*;
use commands::ping;
use commands::get;
use commands::set;
use commands::command::*;

struct GetEvent {
  command: get::Get
}

struct PeekEvent {
  command: get::Get
}

impl Event<Journal, DBResult> for ping::Ping {
  fn precondition(&self, _: &Journal) -> Result<(), Errors> {
    Ok(())
  }

  fn action(&self, _: &mut Journal) -> Result<DBResult, Errors> {
    Ok(DBResult::Pong)
  }

  fn postcondition(&self, _: &Journal) -> Result<(), Errors> {
    Ok(())
  }
}

impl Event<Journal, DBResult> for GetEvent {
  fn precondition(&self, _: &Journal) -> Result<(), Errors> {
    Ok(())
  }

  fn action(&self, state: &mut Journal) -> Result<DBResult, Errors> {
    let res = state.pop();
    Ok(DBResult::Removed(self.command.queue_name().clone(), res))
  }

  fn postcondition(&self, _: &Journal) -> Result<(), Errors> {
    Ok(())
  }
}

impl Event<Journal, DBResult> for PeekEvent {
  fn precondition(&self, _: &Journal) -> Result<(), Errors> {
    Ok(())
  }

  fn action(&self, state: &mut Journal) -> Result<DBResult, Errors> {
    let res = state.peek();
    // This should probably be "Seen" or something, as it is not removed
    Ok(DBResult::Removed(self.command.queue_name().clone(), res))
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
    Ok(DBResult::Inserted(self.queue_name().clone()))
  }

  fn postcondition(&self, _: &Journal) -> Result<(), Errors> {
    Ok(())
  }
}

impl AsEvent<Journal, DBResult> for Command {
  fn as_event(self) -> Box<Event<Journal, DBResult>+Send> {
    match self {
      Command::Ping(p) => box p as Box<Event<Journal, DBResult>+Send>,
      Command::Get(g) => {
        if g.peek() {
          box PeekEvent { command: g } as Box<Event<Journal, DBResult>+Send>
        } else {
          box GetEvent { command: g } as Box<Event<Journal, DBResult>+Send>
        }
      },
      Command::Set(s) => box s as Box<Event<Journal, DBResult>+Send>,
    }
  }
}
