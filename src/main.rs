// mod conditions;
// mod file_reader;
// mod functions;
// mod loops;
mod structs;
// use conditions::{conditional_operator_let_statement, if_else_statement};
// use file_reader::read_from_file;
// use functions::error_handling_match;
// use loops::for_loop_with_range_and_step;
use structs::{Color, Person, User};

fn main() {
    // conditional_operator_let_statement(2);
    // if_else_statement();
    // println!("The results of the loops are: ");
    // for_loop_with_range_and_step(2);
    // error_handling_match("src/main.rs");
    // match read_from_file() {
    //     Ok(v) => println!("{}", v),
    //     Err(e) => println!("Error: {}", e),
    // }
    let human = Person {
        name: String::from("Bob"),
        age: Some(23),
        likes_oranges: true,
    };
    println!("{:?}", human);
    let mut new_user = User::new(
        String::from("Bob"),
        String::from("bob@gmail.com"),
        String::from("google.com"),
    );
    println!("Hi {}", new_user.email);
    new_user.deactivate();
    println!("Is {} active? {}", new_user.email, new_user.active);
    let orange = Color(255, 165, 0);
    println!("Orange is {:?}", orange);
}
