extern crate day06lib;
use day06lib::{count_most_frequent, count_least_frequent};

extern crate util;
use util::get_lines;

fn main() {
    println!("Enter problem lines:");
    let lines = get_lines();
    println!("Most frequent:  {}", count_most_frequent(&lines));
    println!("Least frequent: {}", count_least_frequent(&lines));
}
