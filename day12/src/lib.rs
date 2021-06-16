use aoclib::parse;
use assembunny::{Computer, Instruction, Register};

use std::path::Path;

pub fn part1(input: &Path) -> Result<(), Error> {
    let program: Vec<Instruction> = parse(input)?.collect();
    let mut computer = Computer::from_program(program);
    computer.run();
    println!("value in a after termination: {}", computer[Register::A]);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let program: Vec<Instruction> = parse(input)?.collect();
    let mut computer = Computer::from_program(program);
    computer[Register::C] = 1;
    computer.run();
    println!("value in a after termination: {}", computer[Register::A]);
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
