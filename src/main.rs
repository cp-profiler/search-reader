use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::net::{TcpStream};


fn main() {

    let path = Path::new("/home/maxim/phd/rust-reader/foo.txt");
    
    let mut file = File::open(path).unwrap();

    let mut buf: Vec<u8> = Vec::new();

    file.read_to_end(&mut buf);

    let mut socket = TcpStream::connect("127.0.0.1:6565").unwrap();

    socket.write(&buf);
}
