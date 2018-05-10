extern crate clap;
extern crate byteorder;

use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpStream};

use clap::Arg;

use byteorder::{LittleEndian, ByteOrder};

fn read_file(file: &mut File, debug: bool) {
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

  let mut buf = [0; 500];

  loop {

    if debug {
      println!("Press <ENTER> to send next node\n");
      let mut line = String::new();
      std::io::stdin().read_line(&mut line);
    }

    let msg_len = {
      let mut buf = [0; 4];
      match file.read_exact(&mut buf) {
        Ok(()) => (),
        Err(_) => { break; }
      }
      socket.write(&buf).unwrap();
      LittleEndian::read_u32(&buf)
    };

    let mut buf = {
      let mut buf: Vec<u8> = Vec::new();
      buf.resize(msg_len as usize, 0);
      buf.into_boxed_slice()
    };

    match file.read_exact(&mut buf) {
      Ok(()) => (),
      Err(_) => { break; }
    };

    let size = match socket.write(&buf) {
      Ok(v) => v,
      Err(_) => 0
    };
  }
}

fn main() {

  let args = clap::App::new("Search Reader")
    .version("0.1")
    .about("Reads nodes from a file and sends to CP-Profiler.")
    .author("Maxim Shishmarev")

    .arg(Arg::with_name("file")
      .help("File to read")
      .required(true)
      .index(1))

    .arg(Arg::with_name("debug")
      .short("d")
      .long("debug")
      .help("send nodes one-by-one")
      .takes_value(false))

    .get_matches();

    let path = args.value_of("file").unwrap();

    let mut file = match File::open(&path) {
      Ok(v) => v,
      Err(e) => {
        println!("{:?}", e);
        println!("File location: {:?}", &path);
        std::process::exit(0);
      }

    };

    let debug = args.is_present("debug");

    read_file(&mut file, debug);

}
