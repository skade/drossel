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
use std::str::from_utf8;
use std::io::BufferedStream;

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
          let str = buffer.read_line().unwrap();

          let command: ~Command = from_str(str).unwrap();
          buffer.write(command.execute());
        },
        Err(_) => { fail!("Oha?"); }
      }
    });
  }

  // close the socket server
  drop(acceptor);
}
