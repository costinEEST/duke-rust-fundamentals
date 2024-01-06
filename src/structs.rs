#![allow(dead_code)]

use std::fmt::Debug;

#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: Option<u8>,
    pub likes_oranges: bool,
}

pub struct User {
    pub username: String,
    pub email: String,
    pub uri: String,
    pub active: bool,
}

impl User {
    pub fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }

    // An associative function is one that doesn't require itself.
    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

// Struct as a tuple
pub struct Color(pub u8, pub u8, pub u8);
impl Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Color")
            .field("red", &self.0)
            .field("green", &self.1)
            .field("blue", &self.2)
            .finish()
    }
}
