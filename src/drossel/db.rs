use super::store::*;
use std::collections::dlist::DList;
use super::super::*;
use strand::mutable::Strand;
use strand::mutable::Event;
use strand::strand::Mutable;
use strand::strand;
use strand::errors::{Errors};

pub struct DB {
  queue: Queue
}

impl DB {
  pub fn new() -> DB {
    DB { queue: Queue { state: DList::new() } }
  }

  pub fn execute<S, T, V: Event<BinaryList,S>+Command>(mut self, effect: V) -> Result<S, Errors> {
    self.queue.evolve(&effect)
  }
}


impl Event<BinaryList,&'static str> for Ping {
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