use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

enum TestEnum {
    A,
    B,
    C,
}

impl Distribution<TestEnum> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TestEnum {
        match rng.gen_range(0..=2) {
            0 => TestEnum::A,
            1 => TestEnum::B,
            2 => TestEnum::C,
            _ => unreachable!(),
        }
    }
}

fn main() {
    for _i in 0..=2 {
        match rand::random::<TestEnum>() {
            TestEnum::A => println!("A"),
            TestEnum::B => println!("B"),
            TestEnum::C => println!("C"),
        };
    }
}
