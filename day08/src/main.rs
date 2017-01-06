extern crate day08lib;
use day08lib::TinyScreen;
use day08lib::instruction::Instruction;

extern crate util;
use util::get_lines;

fn main() {
    println!("Enter list of instructions:");
    if let Some(instructions) = get_lines()
        .lines()
        .map(|line| Instruction::parse(line.trim()))
        .collect::<Option<Vec<_>>>() {
        let mut ts = TinyScreen::default();
        for instruction in instructions {
            ts.apply(instruction);
        }
        println!("Lit pixels: {}", ts.num_pixels_lit());
        println!("{}", ts);
    } else {
        println!("Error parsing instructions")
    }
}
