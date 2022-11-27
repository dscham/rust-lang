mod util;
mod server;
mod client;
mod p2p;


fn main() {
    println!("Starting up...");

    p2p::run_heartbeat();
}
