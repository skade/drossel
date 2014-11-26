use strand::mutable::Strand;
use strand::strand::Mutable;
use strand::strand;
use drossel_journal::Journal;

#[deriving(Send)]
pub struct JournaledQueue {
  pub state: Journal,
}

impl strand::Strand<Journal> for JournaledQueue {
  fn new(state: Journal) -> JournaledQueue {
    JournaledQueue { state: state }
  }
}

impl Mutable<Journal> for JournaledQueue {
  fn state<'a>(&'a mut self) -> &'a mut Journal {
    &mut self.state
  }
}

impl JournaledQueue {
  pub fn len(&self) -> uint {
    self.state.len() as uint
  }
}
