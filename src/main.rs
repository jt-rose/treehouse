#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

fn main() {
    let mut visitor_list = vec![
        Visitor::new("Frodo", "safe travels, Frodo!"),
        Visitor::new("Gandalf", "Well met, old wizard"),
        Visitor::new("Aragon", "Hail to the future king!"),
    ];

    loop {
        println!("Hello, what is your name? (leave EMPTY and press enter to quit)");

        let name = get_name();

        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                }
                println!("{} isn't on the visitor list!", capitalize(&name));
                let new_visitor = Visitor::new(&name, "Hello new friend!");
                visitor_list.push(new_visitor);
            }
        }
    }

    println!("the final list of visitors:");
    println!("{:#?}", visitor_list);
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

// set up automatic debug printing
#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn get_name() -> String {
    // ask user for their name
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");

    // return formatted name
    your_name.trim().to_lowercase()
}

// Capitalizes the first character in s.
// referenced from https://nick.groenen.me/notes/capitalize-a-string-in-rust/
pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
