// mod conditions;
// mod enums;
// mod file_reader;
mod file_size_formatter;
// mod functions;
// mod loops;
// mod strings;
// mod structs;
// mod vectors;
// use conditions::{conditional_operator_let_statement, if_else_statement};
// use enums::{total_area, Shape};
// use file_reader::read_from_file;
use file_size_formatter::format_file_size;
// use functions::error_handling_match;
// use loops::for_loop_with_range_and_step;
// use strings::{
//     concatenate_using_format, first_three_characters_from_sentence, loop_over_chars, print_string,
//     print_string_slice,
// };
// use structs::{Color, Person, User};
// use vectors::{
//     get_item, reversed_sentence, reversed_sentence_using_fold, reversed_string,
//     split_string_and_collect_vector,
// };

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

    // let human = Person {
    //     name: String::from("Bob"),
    //     age: Some(23),
    //     likes_oranges: true,
    // };
    // println!("{:?}", human);
    // let mut new_user = User::new(
    //     String::from("Bob"),
    //     String::from("bob@gmail.com"),
    //     String::from("google.com"),
    // );
    // println!("Hi {}", new_user.email);
    // new_user.deactivate();
    // println!("Is {} active? {}", new_user.email, new_user.active);
    // let orange = Color(255, 165, 0);
    // println!("Orange is {:?}", orange);

    // concatenate_using_format("What a beautiful day");
    // loop_over_chars("abracadabra");
    // print_string_slice("A string literal");
    // print_string(String::from("A UTF-8–encoded, growable string"));
    // first_three_characters_from_sentence("the quick brown fox");

    // let _ = get_item(4);
    // split_string_and_collect_vector("remember death");
    // reversed_string("death");
    // reversed_sentence("remember death");
    // reversed_sentence_using_fold("remember death");

    // let shapes = vec![Shape::Circle(0.0), Shape::Square(10.0)];
    // total_area(&shapes);

    format_file_size();
}
