use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::thread;

pub fn start_server() {
    thread::spawn(move || server_main());
}

fn server_main() {
    println!("Setting up Server...");
    let mut socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 6969);
    let mut tcp_listener = TcpListener::bind(socket_addr);
    'try_port: loop {
        match &tcp_listener {
            Ok(listener) =>
                break 'try_port,
            Err(e) => {
                println!("Error: {}", e);
                socket_addr.set_port(socket_addr.port() + 1);
                tcp_listener = TcpListener::bind(socket_addr);
            }
        }
    }

    let tcp_listener = tcp_listener.unwrap();
    println!("Server started on: {}!", tcp_listener.local_addr().unwrap());
    for stream in tcp_listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || handle_connection(&mut stream));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_connection(stream: &mut TcpStream) {
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