use aoclib::parse;
use assembunny::{Computer, Instruction, Register};

use std::path::Path;

/// if this many values match, assume all of them will
const LENGTH_ASSUMPTION: usize = 64;

pub fn part1(input: &Path) -> Result<(), Error> {
    let want_signal = std::array::IntoIter::new([0, 1]).cycle();
    let program: Vec<Instruction> = parse(input)?.collect();
    for a in 0_i32.. {
        let mut computer = Computer::from_program(program.clone());
        let (sender, receiver) = std::sync::mpsc::sync_channel(0);
        computer.set_sender(sender);
        computer[Register::A] = a;

        computer.launch();
        if want_signal
            .clone()
            .take(LENGTH_ASSUMPTION)
            .eq(receiver.into_iter().take(LENGTH_ASSUMPTION))
        {
            println!("value in a producing clock signal: {}", a);
            return Ok(());
        }
    }
    Err(Error::NoSolution)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("no solution found")]
    NoSolution,
}
