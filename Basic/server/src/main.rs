use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

struct Pages {
    filename: String,
    status: String,
}
impl Pages {
    fn new(filename: String, status: String) -> Pages {
        Pages {
            filename: filename,
            status: status,
        }
    }

    fn check_get(page: Pages, mut stream: TcpStream) {
        let content: String = fs::read_to_string(page.filename).unwrap();

        let response: String = format!("{} {}\r\n\r\n{}", page.status, content.len(), content);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn connection_handler(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // println!("Result : {:?}", String::from_utf8_lossy(&buffer[..]));

    let get: &[u8; 16] = b"GET / HTTP/1.1\r\n";
    // Page for Message and Error
    let message: Pages = Pages::new(
        "index.html".to_string(),
        "HTTP/1.1 200 Ok\r\nContent-Length: ".to_string(),
    );
    let error: Pages = Pages::new(
        "404.html".to_string(),
        "HTTP/1.1 404 NOT FOUND\r\nContent-Length: ".to_string(),
    );
    if buffer.starts_with(get) {
        Pages::check_get(message, stream);
    } else {
        Pages::check_get(error, stream);
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
