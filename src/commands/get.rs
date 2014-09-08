#[deriving(Show)]
pub struct Get {
  queue_name: String
}

impl Get {
  pub fn new(queue_name: String) -> Get {
    Get { queue_name: queue_name }
  }

  pub fn queue_name(&self) -> &String {
    &self.queue_name
  }
}

//impl Command for Get {
//  fn execute(&self) -> Vec<u8> {
//    let result = format!("Command: GET queue: {}", self.queue_name);
//    result.as_bytes().to_vec()
//  }
//  fn timeout(&self) { }
//}
