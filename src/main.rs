use std::io;

fn main() {
    let mut inputs = vec![];
    'test: loop {
        println!("Please enter a value:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input = input.trim().to_string();

        if input == "quit" {
            break 'test;
        }

        inputs.push(input.trim_end().to_string());

        println!("You entered those {} inputs:", inputs.len());
        println!("{}", inputs.join(", "));
    }

    println!("Good Bye!");
}
