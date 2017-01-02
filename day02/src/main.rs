extern crate day02lib;
use day02lib::decode;

extern crate util;
use util::get_lines;

fn main() {
    println!("Enter code lines here:");
    let lines = get_lines();
    println!("Decoded: {:?}", decode(&lines));
}
