#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

fn get_name() -> String {
    // ask user for their name
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");

    // return formatted name
    your_name.trim().to_lowercase()
}

fn main() {
    println!("Hello, what is your name?");

    let name = get_name();

    let visitor_list = ["frodo", "gandalf", "aragon"];
    let mut allow_in = false;

    for visitor in &visitor_list {
        if &name == visitor {
            allow_in = true;
        }
    }

    if allow_in {
        println!("Welcome to the club, {}", name);
    } else {
        println!("Scram!");
    }
}
