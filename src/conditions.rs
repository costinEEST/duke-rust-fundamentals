#![allow(dead_code)]

pub fn if_statement() {
    if 1 == 1 {
        println!("Maths is working as expected.");
    }
}

pub fn if_else_statement() {
    let maybe_number: Option<Option<()>> = Some(None);

    if let Some(number) = maybe_number {
        println!("The number is {:?}.", number);
    } else {
        println!("There is no number.");
    }
}

pub fn if_else_if_else_statement() {
    if 1 == 2 {
        println!("Maths is working as expected.");
    } else if 1 == 1 {
        println!("Maths is working as expected.");
    } else {
        println!("Maths is not working as expected.");
    }
}

pub fn switch_statement() {
    let number = 1;
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else."),
    }
}

pub fn nested_conditional_statements() {
    let number = 1;
    if number < 5 {
        if number == 1 {
            println!("One");
        } else {
            println!("Something else.");
        }
    } else {
        println!("Something else.");
    }
}

pub fn conditional_operator_let_statement(i: i32) {
    let condition = i % 2 == 0;
    let result = if condition { "even" } else { "odd" };
    println!("The number {} is {}.", i, result);
}
