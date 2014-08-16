use super::command::Command;

pub struct Set{
  queue_name: String,
  expiration: String,
  payload: Vec<u8>
}

impl Set {
  pub fn new(queue_name: String, expiration: String, payload: Vec<u8>) -> Set {
    Set { queue_name: queue_name, expiration: expiration, payload: payload }
  }

  pub fn queue_name(&self) -> &String {
    &self.queue_name
  }

  pub fn expiration(&self) -> &String {
    &self.expiration
  }

  pub fn payload(&self) -> &Vec<u8> {
    &self.payload
  }
}

//impl Command for Set {
//  fn execute(&self) -> Vec<u8> {
//    let result = format!("Command: SET queue: {}, expiration: {}, payload: {}", self.queue_name, self.expiration, self.payload);
//    result.as_bytes().to_vec()
//  }
//  fn timeout(&self) { }
//}
