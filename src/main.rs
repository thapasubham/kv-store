use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let addr = "127.0.0.1:5000";

    let listener = match TcpListener::bind(addr) {
        Ok(listener) => {
            println!("Listening on {}", addr);
            listener
        }
        Err(err) => {
            eprintln!("Failed to bind: {}", err);
            return;
        }
    };

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(err) => eprintln!("Connection failed: {}", err),
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    println!("New connection from {}", stream.peer_addr().unwrap());

    let mut buf = [0u8; 4096];

    match stream.read(&mut buf) {
        Ok(0) => {
            println!("Client disconnected");
        }
        Ok(n) => {
            println!("Received {} bytes", n);
            println!("{}", String::from_utf8_lossy(&buf[..n]));
        }
        Err(err) => {
            eprintln!("Read failed: {}", err);
            return;
        }
    }

    if let Err(err) = writeln!(stream, "hello") {
        eprintln!("Write failed: {}", err);
    }
}
