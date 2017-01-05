extern crate day06lib;
use day06lib::count_most_frequent;

extern crate util;
use util::get_lines;

fn main() {
    println!("Enter problem lines:");
    let lines = get_lines();
    println!("{}", count_most_frequent(&lines));
}
