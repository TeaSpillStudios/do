use requests::*;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("pc.local:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 1024];

    stream.read(&mut buf).unwrap();

    println!("{}", String::from_utf8_lossy(&buf));

    let output = b"Response test";

    stream.write_all(output).unwrap();
    stream.flush().unwrap();
}
