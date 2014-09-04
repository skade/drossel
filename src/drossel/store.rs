use strand::mutable::Strand;
use strand::strand::Mutable;
use strand::strand;
use drossel::journal::Journal;

#[deriving(Send)]
pub struct Queue {
  pub state: Journal,
}

impl strand::Strand<Journal> for Queue {
  fn new(state: Journal) -> Queue {
    Queue { state: state }
  }
}

impl Mutable<Journal> for Queue {
  fn state<'a>(&'a mut self) -> &'a mut Journal {
    &mut self.state
  }
}
