use std::str::from_utf8;

use super::command::*;
use super::ping;
use super::get;
use super::set;

pub fn get_command(message: Vec<u8>) -> Option<Box<Command>> {
  let split_input: Vec<&[u8]> = message.as_slice().split(|ch| ch == &(' ' as u8) || ch == &('\n' as u8) || ch == &('\r' as u8)).collect();

  match from_utf8(split_input[0]).unwrap().trim() {
    "PING" => { Some(box Ping(ping::Ping)) },
    "GET"  => {
      let split_params: Vec<&[u8]> = split_input[1].split(|ch| ch == &('/' as u8)).collect();
      let open = split_params.iter().any(|a| from_utf8(*a) == Some("open"));
      let close = split_params.iter().any(|a| from_utf8(*a) == Some("cloe"));
      let abort = split_params.iter().any(|a| from_utf8(*a) == Some("abort"));
      let peek = split_params.iter().any(|a| from_utf8(*a) == Some("peek"));
      let get = get::Get::new(
        from_utf8(split_params[0]).unwrap().to_string(),
        None,
        open,
        close,
        abort,
        peek
      );
      Some(box Get(get))
    },
    "SET"  => {
      let set = set::Set::new(
        from_utf8(split_input[1]).unwrap().to_string(),
        from_str(from_utf8(split_input[2]).unwrap()).unwrap(),
        from_str(from_utf8(split_input[3]).unwrap()).unwrap(),
        from_str(from_utf8(split_input[4]).unwrap()).unwrap()
      );
      Some(box Set(set))
    },
    _ => { None }
  }
}

#[cfg(test)]
mod tests {
  use super::get_command;

  #[test]
  fn test_ping() {
    let command = get_command("PING".as_bytes().to_vec());
    assert_eq!("PING", command.unwrap().name())
  }

  #[test]
  fn test_get() {
    let command = get_command("GET test_queue".as_bytes().to_vec());
    assert_eq!("GET", command.unwrap().name())
  }

  #[test]
  fn test_set() {
    let command = get_command("SET test_queue 0 0 24".as_bytes().to_vec());
    assert_eq!("SET", command.unwrap().name())
  }
}
