mod util;
mod server;
mod client;


fn main() {
    println!("Start client or server? (c/s)");

    match util::read_cli_string().as_str() {
        "server" | "s" => server::start_server(),
        "client" | "c" => client::start_client(),
        _ => println!("Invalid input"),
    }
}
