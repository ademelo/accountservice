use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .expect("Failed to bind server to 127.0.0.1:8080");

    println!("Server running at http://127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(error) => eprintln!("Connection failed: {error}"),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    if let Err(error) = stream.read(&mut buffer) {
        eprintln!("Failed to read request: {error}");
        return;
    }

    let request = String::from_utf8_lossy(&buffer);

    let response = if request.starts_with("GET / ") {
        let body = "Hello from accountservice";

        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        )
    } else {
        let body = "Not Found";

        format!(
            "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        )
    };

    if let Err(error) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to write response: {error}");
    }
}