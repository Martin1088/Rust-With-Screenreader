use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn connection_handler(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // println!("Result : {:?}", String::from_utf8_lossy(&buffer[..]));

    let get: &[u8; 16] = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get) {
        let content: String = fs::read_to_string("index.html").unwrap();

        let response: String = format!(
            "HTTP/1.1 200 Ok\r\nContent-Length: {}\r\n\r\n{}",
            content.len(),
            content
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let content: String = fs::read_to_string("404.html").unwrap();

        let response: String = format!(
            "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}",
            content.len(),
            content
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:7878")?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => connection_handler(stream),
            Err(e) => println!("Error"),
        }
    }
    Ok(())
}
