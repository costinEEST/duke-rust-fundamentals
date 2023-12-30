#![allow(dead_code)]

pub fn for_loop_with_range() {
    for i in 1..=5 {
        println!("{}", i);
    }
}

pub fn while_loop() {
    let mut i = 1;

    while i <= 5 {
        println!("{}", i);
        i += 1;
    }
}

pub fn loop_with_break_condition() {
    let mut i = 1;

    loop {
        if i > 5 {
            break;
        }
        println!("{}", i);
        i += 1;
    }
}

pub fn iterator_range() {
    (1..=5).for_each(|i| println!("{}", i));
}

pub fn for_loop_with_range_and_step(s: usize) {
    for i in (1..=5).step_by(s) {
        println!("{}", i);
    }
}

pub fn recursive_range() {
    fn recursive_range(i: i32) {
        if i > 5 {
            return;
        }
        println!("{}", i);
        recursive_range(i + 1);
    }

    recursive_range(1);
}
