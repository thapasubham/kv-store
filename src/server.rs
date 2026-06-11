use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread;

use crate::commands::{CommandOutcome, handle_command};
use crate::database::Database;

pub fn run(addr: &str, db: Arc<RwLock<Database>>) {
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
            Ok(stream) => {
                let db = Arc::clone(&db);

                thread::spawn(move || {
                    handle_client(stream, db);
                });
            }
            Err(err) => {
                eprintln!("Connection failed: {}", err);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, db: Arc<RwLock<Database>>) {
    println!("New connection from {}", stream.peer_addr().unwrap());

    let mut buf = [0u8; 4096];

    loop {
        match stream.read(&mut buf) {
            Ok(0) => {
                println!("Client disconnected");
                return;
            }
            Ok(n) => {
                let input = String::from_utf8_lossy(&buf[..n]);

                let outcome = handle_command(input.as_ref(), &db);

                match outcome {
                    CommandOutcome::Respond(response) => {
                        if let Err(err) = stream.write_all(response.as_bytes()) {
                            println!("Write failed: {}", err);
                            return;
                        }
                    }
                    CommandOutcome::Shutdown => {
                        let _ = stream.write_all(b"BYE\n");
                        println!("Client requested disconnect");
                        return;
                    }
                }
            }
            Err(err) => {
                eprintln!("Read failed: {}", err);
                return;
            }
        }
    }
}
