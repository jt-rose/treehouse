#![warn(clippy::all, clippy::pedantic)]

use std::io::stdin;

fn main() {
    let mut visitor_list = vec![
        Visitor::new("Frodo", VisitorAction::AcceptWithNote { note: String::from("no mead for this one yet")}, 16),
        Visitor::new("Gandalf", VisitorAction::Accept, 99),
        Visitor::new("Aragon", VisitorAction::Accept, 30),
        Visitor::new("Orc", VisitorAction::Refuse, 1),
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
                let new_visitor = Visitor::new(&name, VisitorAction::Probation, 0);
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
    action: VisitorAction,
    age: i32,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i32) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the tavern, {}", &self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the tavern, {}", &self.name);
                println!("{}", note);
                if self.age < 18 {
                    println!("No mead for this young one yet!");
                }

            },
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            VisitorAction::Refuse => println!("We don't serve your kind here, foul monster!")
        }
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
