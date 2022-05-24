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

/// Capitalizes the first character in s.
/// referenced from https://nick.groenen.me/notes/capitalize-a-string-in-rust/
pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
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
        println!("Welcome to the club, {}!", capitalize(&name));
    } else {
        println!("Scram!");
    }
}
