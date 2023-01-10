use std::net::TcpStream;
use std::{
    io,
    io::{Read, Write},
};

fn main() {
    let mut buf = [0; 1024];

    let mut input = String::new();

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
