extern crate day03lib;
use day03lib::{parse_lines_as_usize, count_valid};

extern crate util;
use util::get_lines;

fn main() {
    println!("Enter space-separated positive integer triangles, one per line:");
    let lines = get_lines();
    if let Some(triangles) = parse_lines_as_usize(&lines) {
        println!("{} Valid triangles", count_valid(triangles));
    } else {
        println!("Parse error; no lines found")
    }
}
