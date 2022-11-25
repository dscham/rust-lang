use std::io::{Read, Write};
use std::net::TcpStream;
use crate::util;

pub fn start_client() {
    println!("Setting up  Client...");

    println!("Please enter the server you want to connect: ");
    let host = util::read_cli_string();

    println!("Please enter the port you want to use: ");
    let port = util::read_cli_u16();

    let socket_addr = util::create_socket(host.as_str(), port);
    println!("SocketAddr created");

    let mut tcp_stream = TcpStream::connect(socket_addr).unwrap();
    println!("Client started with socket address: {}!", tcp_stream.peer_addr().unwrap());

    println!("Start chatting. (q to quit): ");
    'client: loop {
        let message = util::read_cli_string();
        if message == "q" {
            break 'client;
        }
        tcp_stream.write(message.as_bytes()).unwrap();

        'response: loop {
            let mut buffer = [0; 1024];
            match tcp_stream.read(&mut buffer) {
                Ok(size) => {
                    println!("|<< {}", String::from_utf8_lossy(&buffer[..size]));
                    break 'response;
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break 'response;
                }
            }
        }
    }
}