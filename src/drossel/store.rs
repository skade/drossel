use strand::mutable::Strand;
use strand::mutable::Event;
use strand::strand::Mutable;
use strand::strand;
use strand::errors::{Errors};
use std::collections::dlist::DList;
use std::collections::Deque;

pub type BinaryList = DList<Vec<u8>>;

#[deriving(Send)]
pub struct Queue {
  pub state: BinaryList,
}

impl strand::Strand<BinaryList> for Queue {
  fn new(state: BinaryList) -> Queue {
    Queue { state: state }
  }
}

impl Mutable<BinaryList> for Queue {
  fn state<'a>(&'a mut self) -> &'a mut BinaryList {
    &mut self.state
  }
}
