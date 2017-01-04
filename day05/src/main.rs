extern crate day05lib;
use day05lib::GetPassword;

use std::io::{Write, stdout};

fn main() {
    const INPUT: &'static str = "ugkcyxxp";
    println!("Hashing...");
    for pw_char in GetPassword::new(INPUT).take(8) {
        print!("{}", pw_char);
        stdout().flush().expect("failed to flush stdout");
    }
    println!();
}
