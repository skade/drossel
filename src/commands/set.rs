use super::command::Command;

pub struct Set{
  queue_name: String,
  payload: Vec<u8>
}

impl Set {
  pub fn new(queue_name: String, payload: Vec<u8>) -> Set {
    Set { queue_name: queue_name, payload: payload }
  }

  pub fn queue_name(&self) -> &String {
    &self.queue_name
  }

  pub fn payload(&self) -> &Vec<u8> {
    &self.payload
  }
}
