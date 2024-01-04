#![allow(dead_code)]

use std::io::BufRead;

// A unit function is one that does not return a value
pub fn process_numbers(numbers: &[i32]) {
    let mut sum = 0;

    for number in numbers {
        sum += number
    }

    println!("The sum of the numbers is: {}", sum);

    if sum % 2 == 0 {
        println!("The sum is even");
    } else {
        println!("The sum is odd");
    }
}

pub fn split_string(s: String, delimiter: char, field: usize) -> String {
    let parts: Vec<&str> = s.split(delimiter).collect();
    let result = parts.get(field);

    result.expect("Something went wrong").to_string()
}

// no variadic arguments in Rust
pub fn sum(numbers: &[i32]) -> i32 {
    let mut sum = 0;

    for number in numbers {
        sum += number
    }

    sum
}

/**
 * Borrowing is the mechanism by which Rust allows you to lend ownership of a variable to a function or another part of your program without actually transferring ownership of the variable.
 * When you borrow a variable, you're essentially saying "I want to use this variable for a little while, but I promise I won't modify it."
 * */
pub fn own_vec(vector: &mut Vec<i32>) -> Vec<i32> {
    let new_vector = Vec::new();
    vector.push(1);
    new_vector
}

pub fn error_handling_match(path: &str) {
    let file = std::fs::File::open(path);
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => panic!("File not found: {:?}", error),
            // instead of panicking when an error occurs, better return the error
            _ => panic!("Problem opening the file: {:?}", error),
        },
    };

    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}
