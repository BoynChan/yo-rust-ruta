use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use web_server::ThreadPool;

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    let valid_head = "GET / HTTP";
    let (status_line, filename) = if buf.starts_with(valid_head.as_bytes()) {
        ("200 OK", "index.html")
    } else {
        ("404 Not Found", "404.html")
    };
    let html = fs::read_to_string(filename).unwrap();
    let resp = format!("HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}", status_line, html.len(), html);
    stream.write(resp.as_bytes()).unwrap();

    stream.flush().unwrap();
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(move ||{
                    handle_connection(stream);
                })
            }
            Err(e) => { /* connection failed */ }
        }
    }
}
