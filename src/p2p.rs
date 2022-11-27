use std::{thread, time};
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::str::FromStr;
use crate::util;

pub struct Peer {
    addr: SocketAddr
}

pub fn run_heartbeat() {
    println!("Setting up heartbeat...");

    let socket = UdpSocket::bind("0.0.0.0:42069").expect("Could not bind UDP socket");
    socket.join_multicast_v4(&Ipv4Addr::from_str("224.0.0.1").unwrap(), &Ipv4Addr::UNSPECIFIED).expect("Could not join multicast group");
    socket.set_multicast_loop_v4(false).unwrap();
    let socket_copy = socket.try_clone().unwrap();
    let mut threads = vec![];

    /*match util::read_cli_u16() {
        1 => threads.push(thread::spawn(move || heartbeat_sender(socket))),
        _ => threads.push(thread::spawn(move || heartbeat_receiver(socket_copy)))
    }*/

    threads.push(thread::spawn(move || heartbeat_sender(socket)));
    threads.push(thread::spawn(move || heartbeat_receiver(socket_copy)));

    for thread in threads {
        thread.join().unwrap();
    }
}

fn heartbeat_sender(socket: UdpSocket) {
    loop {
        println!("tik");
        socket.send_to("HELLO".as_bytes(), "224.0.0.1:42069").expect("Failed to send data");
        thread::sleep(time::Duration::from_secs(10));
        println!("tok");
    }
}

fn heartbeat_receiver(socket: UdpSocket) {
    loop {
        println!("ba");
        let mut buffer = [0; 1024];
        match socket.peek_from(&mut buffer) {
            Ok((size, peer)) => {
                let mut buffer = vec![0; size];
                socket.recv_from(&mut buffer).expect("Failed to receive data");
                println!("Received: {}", String::from_utf8_lossy(&buffer));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
        println!("dum");
    }
}
