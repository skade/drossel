use strand::mutable::Strand;
use strand::strand::Mutable;
use strand::strand;
use std::collections::dlist::DList;

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
