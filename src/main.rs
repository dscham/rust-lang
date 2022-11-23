use std::io;

struct Rectangle {
    width: u32,
    height: u32,
}

struct Area(u32, String);

fn main() {
    println!("Rectangle Area Calculator");
    println!("Please enter the width of the rectangle:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let width: u32 = input.trim().parse().expect("Please type a number!");
    input.clear();

    println!("Please enter the height of the rectangle:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let height: u32 = input.trim().parse().expect("Please type a number!");

    let rect = Rectangle { width, height };
    let area = Area(rect.width * rect.height, "qm".to_string());

    println!("The area of the rectangle is {}{}", area.0, area.1);
}
