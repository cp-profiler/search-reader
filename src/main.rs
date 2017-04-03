use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpStream};


fn main() {

    let args : Vec<String> = std::env::args().collect();

    if args.len() != 2 {
      println!("Usage: .\\rust-reader <path>");
      std::process::exit(0);
    }

    let path = args[1].clone();

    let mut file = match File::open(path) {
      Ok(v) => v,
      Err(e) => {
        println!("{:?}", e);
        std::process::exit(0);
      }

    };

    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let mut socket = match TcpStream::connect("127.0.0.1:6565") {
      Ok(v) => v,
      Err(e) => {
        if e.kind() == std::io::ErrorKind::ConnectionRefused {
          println!("Connection refused. Is profiler running?");
        } else {
          println!("{:?}", e.kind());
        }
        std::process::exit(0);
      }
    };

    /// Can't I just stream from a file to a socket?
    socket.write(&buf).unwrap();
}
