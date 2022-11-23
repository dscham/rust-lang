use std::fmt;
use std::io;

struct Area {
    area: u32,
    unit: String,
}

struct Edge {
    length: u32,
    unit: String,
}

struct Rectangle {
    width: Edge,
    height: Edge,
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.length, self.unit)
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.area, self.unit)
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rectangle[Width: {}, Height: {}, Area: {}]",
               self.width,
               self.height,
               Area::of(self)
        )
    }
}

impl Area {
    fn of(rectangle: &Rectangle) -> Area {
        Area {
            area: rectangle.width.length * rectangle.height.length,
            unit: format!("{}*{}", rectangle.width.unit, rectangle.height.unit),
        }
    }
}

fn main() {
    println!("Rectangle Area Calculator");

    println!("Unit:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let unit: String = input.trim().to_string();
    input.clear();

    println!("Width:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let width: u32 = input.trim().parse().expect("Please type a number!");
    input.clear();

    println!("Height:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let height: u32 = input.trim().parse().expect("Please type a number!");

    let rect = Rectangle {
        width: Edge {
            length: width,
            unit: unit.clone(),
        },
        height: Edge {
            length: height,
            unit: unit.clone(),
        },
    };

    println!("{}", rect);
}
