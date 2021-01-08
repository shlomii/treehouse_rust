//With help of the great book hands on rust
#![warn(clippy::all, clippy::pedantic)]
use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }
    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Generic message: Welcome to the tree house."),
            VisitorAction::Probation => println!("Probation"),
            VisitorAction::AcceptWithNote { note } => println!("{}", note),
            _ => println!("Go away!"),
        }
    }
}

fn main() {
    println!("Hello, what's your name?");
    let mut visitor_list = vec![
        Visitor::new(
            "bert",
            VisitorAction::AcceptWithNote {
                note: "Give them a taco".to_string(),
            },
            34,
        ),
        Visitor::new(
            "steve",
            VisitorAction::Accept,
            19,
        ),
        Visitor::new("fred1", VisitorAction::Refuse, 34),
        Visitor::new("fred2", VisitorAction::Probation, 34),
    ];

    println!("{:#?}", visitor_list);
    println!("{:?}", visitor_list);
    loop {
        let name = what_is_your_name();
        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the visitor list.", name);
                    visitor_list.push(Visitor::new(&name, VisitorAction::Refuse, 23))
                }
            }
        }
    }
    println!("The final list of visitors:");
    println!("{:#?}", visitor_list);
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}
