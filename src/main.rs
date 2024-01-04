mod conditions;
mod functions;
mod loops;
use conditions::{conditional_operator_let_statement, if_else_statement};
use functions::error_handling_match;
use loops::for_loop_with_range_and_step;

fn main() {
    conditional_operator_let_statement(2);
    if_else_statement();
    println!("The results of the loops are: ");
    for_loop_with_range_and_step(2);
    error_handling_match("src/main.rs");
}
