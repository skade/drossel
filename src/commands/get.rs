#[deriving(Show)]
pub struct Get {
  queue_name: String,
  wait: Option<u32>,
  open: bool,
  close: bool,
  abort: bool,
  peek: bool
}

impl Get {
  pub fn new(queue_name: String,
             wait: Option<u32>,
             open: bool,
             close: bool,
             abort: bool,
             peek: bool) -> Get {
    Get { queue_name: queue_name,
          wait: wait,
          open: open,
          close: close,
          abort: abort,
          peek: peek }
  }

  pub fn queue_name(&self) -> &String {
    &self.queue_name
  }

  pub fn open(&self) -> bool {
    self.open
  }

  pub fn close(&self) -> bool {
    self.close
  }

  pub fn abort(&self) -> bool {
    self.abort
  }

  pub fn peek(&self) -> bool {
    self.peek
  }

  pub fn wait(&self) -> Option<u32> {
    self.wait
  }
}
