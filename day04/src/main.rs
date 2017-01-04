extern crate day04lib;
use day04lib::sum_valid_sectors;

extern crate util;
use util::get_lines;

fn main() {
    println!("Enter room lines here:");
    let lines = get_lines();
    println!("Sum of valid sectors: {:?}", sum_valid_sectors(&lines));
}
