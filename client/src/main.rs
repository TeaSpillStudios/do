use requests::*;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut buf = [0; 1024];

    match TcpStream::connect("pc.local:7878") {
        Ok(mut stream) => {
            stream.read(&mut buf).unwrap();

            stream.write(b"Response").unwrap();
            stream.flush().unwrap();
        }
        Err(err) => {
            println!("Error: {err}");
        }
    }
}
