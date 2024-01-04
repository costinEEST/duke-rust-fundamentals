// mod conditions;
mod file_reader;
// mod functions;
// mod loops;
// use conditions::{conditional_operator_let_statement, if_else_statement};
use file_reader::read_from_file;
// use functions::error_handling_match;
// use loops::for_loop_with_range_and_step;

fn main() {
    // conditional_operator_let_statement(2);
    // if_else_statement();
    // println!("The results of the loops are: ");
    // for_loop_with_range_and_step(2);
    // error_handling_match("src/main.rs");
    match read_from_file() {
        Ok(v) => println!("{}", v),
        Err(e) => println!("Error: {}", e),
    }
}
