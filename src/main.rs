#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

fn get_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name
}

fn main() {
    println!("Hello, what is your name?");

    let name = get_name();

    println!("Hello {}", &name);
}
