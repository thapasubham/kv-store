use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::Bytes;

let mut db: HashMap<String, String> = HashMap::new();
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


fn handle_command(
    input: &str,
    db: &mut HashMap<String, String>,
) -> String {
    let parts: Vec<&str> =
        input.trim().split_whitespace().collect();

    if parts.len() == 3 && parts[0].eq_ignore_ascii_case("SET") {
        db.insert(parts[1].to_string(), parts[2].to_string());

        return "OK\n".to_string();
    }

    "ERR\n".to_string()
}
