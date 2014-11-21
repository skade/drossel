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
      Command::Ping(_) => "PING",
      Command::Get(_) => "GET",
      Command::Set(_) => "SET"
    }
  }

  pub fn read_more_data(&self) -> Option<uint> {
    match *self {
      Command::Ping(_) => None,
      Command::Set(ref s) => Some(s.length()),
      Command::Get(_) => None
    }
  }

  pub fn set_payload(&mut self, payload: &[u8]) {
    match *self {
      Command::Set(ref mut s) => { s.set_payload(payload) },
      _ => ()
    }
  }
}
