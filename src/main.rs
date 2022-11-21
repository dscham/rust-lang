use std::io;

fn main() {
    println!("Hello... Who are you?");

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    name = name.trim_end().to_string();

    println!("Hello, {}!", name);
}
