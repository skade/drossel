use super::store::{Queue};
use std::collections::dlist::DList;

pub struct DB {
  queue: Queue
}

impl DB {
  fn new() -> DB {
    DB { queue: Queue { state: DList::new() } }
  }
}