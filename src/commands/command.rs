use super::ping;
use super::get;
use super::set;

#[deriving(Show)]
pub enum Command {
  Ping(ping::Ping),
  Get(get::Get),
  Set(set::Set)
}

impl Command {
  pub fn name(&self) -> &'static str {
    match *self {
      Ping(_) => "PING",
      Get(_) => "GET",
      Set(_) => "SET"
    }
  }

  pub fn read_more_data(&self) -> Option<uint> {
    match *self {
      Ping(_) => None,
      Set(ref s) => Some(s.length()),
      Get(_) => None
    }
  }

  pub fn set_payload(&mut self, payload: &[u8]) {
    match *self {
      Set(ref mut s) => { s.set_payload(payload) },
      _ => ()
    }
  }
}
