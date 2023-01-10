use requests::*;
use ron::{de::from_str, ser::to_string};
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let request = Requests::Send(Request {
        operation: Operation::Add,
        section: Some(String::from("A test section")),
        name: Some(String::from("Test")),
        data: Some(String::from("A test task")),
    });

    let data = to_string(&request).unwrap();

    let mut buf = [0; 1024];

    let mut stream = TcpStream::connect("pc.local:7878").unwrap();

    // stream.read(&mut buf).unwrap();

    stream.write_all(data.as_bytes()).unwrap();
    stream.flush().unwrap();
}
