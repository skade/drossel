use strand::mutable::Strand;
use strand::mutable::Event;
use strand::strand::Mutable;
use strand::strand;
use strand::errors::{Errors};
use std::collections::dlist::DList;
use std::collections::Deque;

pub type BinaryList = DList<Vec<u8>>;

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

pub struct Insert {
  binary: Vec<u8>
}

impl Insert {
  fn new(binary: Vec<u8>) -> Insert {
    Insert { binary: binary }
  }
}

impl Event<BinaryList,()> for Insert {
  fn precondition(&self, _: &BinaryList) -> Result<(), Errors> {
    Ok(())
  }

  fn action(&self, state: &mut BinaryList) -> Result<(), Errors> {
    state.push(self.binary.clone());
    Ok(())
  }

  fn postcondition(&self, _: &BinaryList) -> Result<(), Errors> {
    Ok(())
  }
}

pub struct Remove;

impl Remove {
  fn new() -> Remove {
    Remove
  }
}

impl Event<BinaryList, Option<Vec<u8>>> for Remove {
  fn precondition(&self, _: &BinaryList) -> Result<(), Errors> {
    Ok(())
  }

  fn action(&self, state: &mut BinaryList) -> Result<Option<Vec<u8>>, Errors> {
    Ok(state.pop_front())
  }

  fn postcondition(&self, _: &BinaryList) -> Result<(), Errors> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::{Insert,Remove,BinaryList,Queue};
  use std::collections::dlist::DList;
  use strand::mutable::Event;
  use strand::mutable::Strand;
  use strand::branchable::Branchable;
  use strand::strand::{Mutable};
  use strand::strand;
  use strand::errors::{Errors};

  #[test]
  fn test_insert() {
    let mut q = Queue { state: DList::new() };
    let i = Insert::new("fooobar".as_bytes().to_vec());
    q.evolve(&i);
    assert_eq!(1, q.state.len())
  }

  #[test]
  fn test_remove() {
    let mut q = Queue { state: DList::new() };
    let i = Insert::new("fooobar".as_bytes().to_vec());
    let r = Remove;
    q.evolve(&i);
    let result: Result<Option<Vec<u8>>, Errors> = q.evolve(&r);
    assert!(result.is_ok());
    assert_eq!(0, q.state.len());
    match result {
      Ok(optional) => match optional {
        Some(vec) => { assert_eq!("fooobar".as_bytes().to_vec(), vec)}
        None => { fail!("We don't expect None as a result")}
      },
      _ => { fail!("We expect the result to be Ok.")}
    }
  }
}