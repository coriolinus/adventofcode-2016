
extern crate day01lib;
use day01lib::parse;

extern crate util;
use util::get_line;

fn main() {

    let directions = parse(&get_line().trim());
    println!("{:?}", directions);
}
