use std::io::{Read, Write};
use std::net::TcpListener;
use crate::util;

pub fn start_server() {
    println!("Setting up Server...");

    println!("Please enter the port you want the server to listen on: ");
    let port = util::read_cli_u16();

    let socket_addr = util::create_socket("0.0.0.0", port);
    println!("SocketAddr created");

    let tcp_listener = TcpListener::bind(socket_addr).unwrap();
    println!("Server started on: {}!", tcp_listener.local_addr().unwrap());

    for stream in tcp_listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                'server: loop {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(size) => {
                            if size == 0 {
                                break 'server;
                            }
                            println!("|<< {}", String::from_utf8_lossy(&buffer[..size]));
                            stream.write("ack".as_bytes()).unwrap();
                            println!(">>| ack");
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                            break 'server;
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}