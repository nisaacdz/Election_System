use std::any::Any;

use electionapp::{Detail, Voter};

pub struct NameDetail {
    name: String,
}

impl Detail for NameDetail {
    fn title(&self) -> &str {
        "Name"
    }

    fn body(&self) -> &dyn Any {
        &self.name
    }
}

pub struct AgeDetail {
    age: u8,
}

impl Detail for AgeDetail {
    fn title(&self) -> &str {
        "Age"
    }

    fn body(&self) -> &dyn Any {
        &self.age
    }
}

fn main() {
    let name_detail = NameDetail {
        name: "Alice".to_string(),
    };

    let age_detail = AgeDetail { age: 30 };

    let voter = Voter {
        details: vec![Box::new(name_detail), Box::new(age_detail)],
        public_key: vec![1, 2, 3, 4],
    };

    if let Some(name) = voter.get_detail::<String>("Name") {
        println!("Name: {}", name);
    }

    if let Some(age) = voter.get_detail::<u8>("Age") {
        println!("Age: {}", age);
    }
}

