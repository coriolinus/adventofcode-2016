use day11::{goalseek, input};

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
