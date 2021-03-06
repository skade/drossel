use std::str::from_utf8;

use super::command::*;
use super::ping;
use super::get;
use super::set;

pub fn get_command(message: Vec<u8>) -> Option<Box<Command>> {
  let split_input: Vec<&[u8]> = message.as_slice().split(|ch| ch == &(' ' as u8) || ch == &('\n' as u8) || ch == &('\r' as u8)).collect();

  match from_utf8(split_input[0]).unwrap().trim() {
    "PING" => { Some(box Command::Ping(ping::Ping)) },
    "GET"  => {
      let split_params: Vec<&[u8]> = split_input[1].split(|ch| ch == &('/' as u8)).collect();
      let open = split_params.iter().any(|a| from_utf8(*a) == Some("open"));
      let close = split_params.iter().any(|a| from_utf8(*a) == Some("close"));
      let abort = split_params.iter().any(|a| from_utf8(*a) == Some("abort"));
      let peek = split_params.iter().any(|a| from_utf8(*a) == Some("peek"));
      let wait_string = split_params.iter().find(|a| a[0] == 't' as u8 && a[1] == '=' as u8);
      let wait = match wait_string {
        Some(string) => {
          let time = string.split(|ch| ch == &('=' as u8)).last();
          from_str(from_utf8(time.unwrap()).unwrap())
        },
        _ => None
      };
      let get = get::Get::new(
        from_utf8(split_params[0]).unwrap().to_string(),
        wait,
        open,
        close,
        abort,
        peek
      );
      Some(box Command::Get(get))
    },
    "SET"  => {
      let set = set::Set::new(
        from_utf8(split_input[1]).unwrap().to_string(),
        from_str(from_utf8(split_input[2]).unwrap()).unwrap(),
        from_str(from_utf8(split_input[3]).unwrap()).unwrap(),
        from_str(from_utf8(split_input[4]).unwrap()).unwrap()
      );
      Some(box Command::Set(set))
    },
    _ => { None }
  }
}

#[cfg(test)]
mod tests {
  use super::get_command;
  use super::super::command::{Command};

  #[test]
  fn test_ping() {
    let command = get_command("PING".as_bytes().to_vec());
    assert_eq!("PING", command.unwrap().name())
  }

  #[test]
  fn test_get() {
    let command = get_command("GET test_queue/open/t=100".as_bytes().to_vec()).unwrap();
    assert_eq!("GET", command.name())
    match *command {
      Command::Get(ref g) => {
        assert!(g.open());
        assert_eq!(g.wait(), Some(100));
      },
      _ => panic!("GET open failed")
    }
  }

  #[test]
  fn test_set() {
    let command = get_command("SET test_queue 0 0 24".as_bytes().to_vec());
    assert_eq!("SET", command.unwrap().name())
  }
}
