use std::io;
use std::io::prelude::*;

pub fn get_line() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Could not read from Stdin");
    buffer
}

pub fn get_lines() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Could not read from Stdin");
    buffer
}
