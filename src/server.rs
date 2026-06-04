use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::commands::handle_command;
use crate::database::Database;

pub fn run(addr: &str, db: &mut Database) {
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
            Ok(stream) => handle_client(stream, db),
            Err(err) => eprintln!("Connection failed: {}", err),
        }
    }
}

fn handle_client(mut stream: TcpStream, db: &mut Database) {
    println!("New connection from {}", stream.peer_addr().unwrap());

    let mut buf = [0u8; 4096];

    match stream.read(&mut buf) {
        Ok(0) => {
            println!("Client disconnected");
        }
        Ok(n) => {
            println!("Received {} bytes", n);
            let input = String::from_utf8_lossy(&buf[..n]);
            let response = handle_command(&input, db);

            let _ = stream.write_all(response.as_bytes());
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
