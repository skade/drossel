use strand::mutable::Strand;
use strand::mutable::Event;
use strand::strand::Mutable;
use strand::strand;
use strand::errors::{Errors, PreConditionNotMet, PostConditionNotMet};
use std::collections::dlist::DList;
use std::collections::Deque;


type BinaryList = DList<Vec<u8>>;

struct Queue {
  state: BinaryList,
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

struct Insert {
  binary: Vec<u8>
}

impl Event<BinaryList> for Insert {
  fn precondition(&self, _: &BinaryList) -> Result<(), Errors> {
    Ok(())
  }

  fn action(&self, state: &mut BinaryList) -> Result<(), Errors> {
    state.push_back(self.binary.clone());
    Ok(())
  }

  fn postcondition(&self, _: &BinaryList) -> Result<(), Errors> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn truthy() {
    assert!(true)
  }
}
