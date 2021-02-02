#[derive(Debug)]
enum Gender {
    Male,
    Female,
}

#[derive(Debug)]
struct User {
    username: String,
    first_name: String,
    last_name: String,
    gender: Gender
}

impl User {
    
    fn info(&self) -> String {

        let symbol: char = match self.gender {
            Male => '♂',
            Female => '♀',
        };

        format!("{} {} {} [@{}]", symbol, self.first_name, self.last_name, self.username)
    }
}

use Gender::*;

fn main() {

    let user_1: User = User {
        username: String::from("muhammad"),
        first_name: String::from("Muhammad"),
        last_name: String::from("Najimov"),
        gender: Male,
    };

    let user_2: User = User {
        username: String::from("aisha"),
        first_name: String::from("Aisha"),
        last_name: String::from("Najimova"),
        gender: Female,
    };

    println!("{}", user_1.info());
    println!("{}", user_2.info());
}
