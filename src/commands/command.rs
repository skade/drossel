use super::ping;
use super::get;
use super::set;

pub enum Command {
  Ping(ping::Ping),
  Get(get::Get),
  Set(set::Set)
}

impl Command {
  pub fn name(self) -> &'static str {
    match self {
      Ping(_) => "PING",
      Get(_) => "GET",
      Set(_) => "SET"
    }
  }
}
//pub trait Command {
//  fn execute(&self) -> Vec<u8>;
//  fn timeout(&self);
//}
//
//pub trait AcknowledgeableCommand {
//  fn acknowledge(&self);
//}