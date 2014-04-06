#![crate_type = "bin"]
#![crate_id = "drossel#0.0.1"]
#![feature(globs,phase)]
#![phase(syntax, link)] extern crate log;

extern crate green;
extern crate rustuv;
extern crate drossel;

use std::io::net::tcp::TcpListener;
use std::io::net::ip::{Ipv4Addr, SocketAddr};
use std::io::{Acceptor, Listener};
use drossel::*;
use std::io::BufferedStream;
use std::str::from_utf8;

#[start]
fn start(argc: int, argv: **u8) -> int {
    green::start(argc, argv, rustuv::event_loop, main)
}

fn main() {
  let addr = SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 7890 };
  let listener = TcpListener::bind(addr);

  // bind the listener to the specified address
  let mut acceptor = listener.listen();

  // accept connections and process them
  for stream in acceptor.incoming() {
    spawn(proc() {
      match stream {
        Ok(conn) => {
          let mut buffer = BufferedStream::new(conn);
          let input = buffer.read_until('\n' as u8).unwrap();
          let split_input: ~[&[u8]] = input.split(|ch| ch == &(' ' as u8)).collect();
          let cmd = from_utf8(split_input[0]).unwrap();

          let command: ~Command = from_str(cmd).unwrap();
          buffer.write(command.execute(split_input.tail()));
        },
        Err(_) => { fail!("Oha?"); }
      }
    });
  }

  // close the socket server
  drop(acceptor);
}
