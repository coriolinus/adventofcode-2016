extern crate day11lib;
use day11lib::{goalseek, input};

fn main() {
    let s = input();
    println!("initial:");
    println!("{}", s);

    if let Some(steps) = goalseek(s) {
        println!("found solution in {} steps", steps);
    } else {
        println!("could not find solution");
    }
}
