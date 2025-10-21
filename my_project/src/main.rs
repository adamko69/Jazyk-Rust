use std::string;

struct Person {
    name: String,
    age: i32,
}

fn main() {
    println!("Hello, world!");

    let x = 4;

    println!("{}", x);
    println!("{}", x);

    let y = x;

    println!("{}", y);
}
