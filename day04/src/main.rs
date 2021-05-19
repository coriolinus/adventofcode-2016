use day04lib::{sum_valid_sectors, find_np_lines, validate, decrypt};

use util::get_lines;

fn main() {
    println!("Enter room lines here:");
    let lines = get_lines();
    println!("Sum of valid sectors: {:?}", sum_valid_sectors(&lines));
    println!();
    println!("Rooms mentioning the North Pole:");
    for (name, sector) in find_np_lines(&lines) {
        println!("  {}  {}", name, sector);
    }
    println!();
    println!("All valid rooms:");
    for line in lines.lines() {
        if validate(line) {
            println!("{} ({})", decrypt(line).unwrap(), line);
        }
    }
}
