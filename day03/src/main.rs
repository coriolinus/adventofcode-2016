use day03::{parse_lines_as_usize, count_valid, parse_lines_vertical_as_usize};

use util::get_lines;

fn main() {
    println!("Enter space-separated positive integer triangles, one per line:");
    let lines = get_lines();
    if let Some(triangles) = parse_lines_as_usize(&lines) {
        println!("{} Valid horizontal triangles", count_valid(triangles));
    } else {
        println!("Parse error; no horizontal lines found");
    }
    if let Some(triangles) = parse_lines_vertical_as_usize(&lines) {
        println!("{} Valid vertical triangles", count_valid(triangles));
    } else {
        println!("Parse error; no vertical lines found");
    }
}
