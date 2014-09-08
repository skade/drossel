#[deriving(Show)]
pub struct Set {
  queue_name: String,
  flags: u32,
  expiration: u32,
  length: uint,
  payload: Vec<u8>
}

impl Set {
  pub fn new(queue_name: String,
             flags: u32,
             expiration: u32,
             length: uint) -> Set {
    Set { queue_name: queue_name,
          payload: Vec::with_capacity(length),
          length: length,
          flags: flags,
          expiration: expiration }
  }

  pub fn queue_name(&self) -> &String {
    &self.queue_name
  }

  pub fn payload(&self) -> &Vec<u8> {
    &self.payload
  }

  pub fn set_payload(&mut self, payload: &[u8]) {
    self.payload.push_all(payload);
  }

  pub fn length(&self) -> uint {
    self.length
  }

  pub fn expiration(&self) -> u32 {
    self.expiration
  }
}
