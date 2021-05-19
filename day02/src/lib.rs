//! Advent of Code - Day 02 Instructions
//!
//! Bathroom Security
//!
//! You arrive at Easter Bunny Headquarters under cover of darkness. However, you left in such a
//! rush that you forgot to use the bathroom! Fancy office buildings like this one usually have
//! keypad locks on their bathrooms, so you search the front desk for the code.
//!
//! "In order to improve security," the document you find says, "bathroom codes will no longer
//! be written down. Instead, please memorize and follow the procedure below to access the
//! bathrooms."
//!
//! The document goes on to explain that each button to be pressed can be found by starting on
//! the previous button and moving to adjacent buttons on the keypad: U moves up, D moves down,
//! L moves left, and R moves right. Each line of instructions corresponds to one button,
//! starting at the previous button (or, for the first line, the "5" button); press whatever
//! button you're on at the end of each line. If a move doesn't lead to a button, ignore it.
//!
//! You can't hold it much longer, so you decide to figure out the code as you walk to the
//! bathroom. You picture a keypad like this:
//!
//! ```notrust
//! 1 2 3
//! 4 5 6
//! 7 8 9
//! ```
//!
//! Suppose your instructions are:
//!
//! ```notrust
//! ULL
//! RRDDD
//! LURDL
//! UUUUD
//! ```
//!
//! You start at "5" and move up (to "2"), left (to "1"), and left (you can't, and stay on "1"),
//! so the first button is 1.
//!
//! Starting from the previous button ("1"), you move right twice (to "3") and then down three
//! times (stopping at "9" after two moves and ignoring the third), ending up with 9.
//!
//! Continuing from "9", you move left, up, right, down, and left, ending with 8.
//!
//! Finally, you move up four times (stopping at "2"), then down once, ending with 5.
//!
//! So, in this example, the bathroom code is 1985.
//!
//! Your puzzle input is the instructions from the document you found at the front desk.
//! What is the bathroom code?

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

impl Instruction {
    pub fn from_char(ch: char) -> Option<Instruction> {
        use Instruction::*;

        match ch {
            'u' | 'U' => Some(Up),
            'd' | 'D' => Some(Down),
            'l' | 'L' => Some(Left),
            'r' | 'R' => Some(Right),
            _ => None,
        }
    }

    // disable clippy here because it's too much hassle to come up with a better name
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Vec<Instruction>> {
        s.trim().chars().map(Instruction::from_char).collect()
    }
}

pub type Keypad = &'static [&'static [Option<char>]];

const KEYPAD_ORTHO: Keypad = &[
    &[Some('1'), Some('2'), Some('3')],
    &[Some('4'), Some('5'), Some('6')],
    &[Some('7'), Some('8'), Some('9')],
];

const KEYPAD_DIAG: Keypad = &[
    &[None, None, Some('1'), None, None],
    &[None, Some('2'), Some('3'), Some('4'), None],
    &[Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
    &[None, Some('A'), Some('B'), Some('C'), None],
    &[None, None, Some('D'), None, None],
];

/// Represents a key on a keypad
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Key {
    x: usize,
    y: usize,
    pad: Keypad,
}

impl Key {
    pub fn center_on(key: char, keypad: Keypad) -> Option<Key> {
        for (y, row) in keypad.iter().enumerate() {
            for (x, maybe_key) in row.iter().enumerate() {
                if *maybe_key == Some(key) {
                    return Some(Key { x, y, pad: keypad });
                }
            }
        }
        None
    }

    pub fn shift(&mut self, inst: Instruction) {
        use Instruction::*;

        match inst {
            Up => {
                if self.y > 0 && self.pad[self.y - 1][self.x].is_some() {
                    self.y -= 1
                }
            }
            Down => {
                if self.y < self.pad.len() - 1 && self.pad[self.y + 1][self.x].is_some() {
                    self.y += 1;
                }
            }
            Left => {
                if self.x > 0 && self.pad[self.y][self.x - 1].is_some() {
                    self.x -= 1;
                }
            }
            Right => {
                if self.x < self.pad[self.y].len() - 1 && self.pad[self.y][self.x + 1].is_some() {
                    self.x += 1;
                }
            }
        }
    }

    pub fn shift_many(&mut self, insts: &[Instruction]) {
        for inst in insts {
            self.shift(*inst);
        }
    }

    pub fn char(&self) -> char {
        self.pad[self.y][self.x].expect("can't have a key without a char")
    }
}

/// Parse a number of lines into a code.
pub fn decode_on(reader: impl BufRead, keypad: Keypad) -> Result<String, Error> {
    let mut key = Key::center_on('5', keypad).ok_or(Error::BadKeypad)?;
    let mut out = String::new();

    for line in reader.lines() {
        let line = line?;
        let instructions = Instruction::from_str(&line).ok_or(Error::UnknownInstruction)?;
        key.shift_many(&instructions);
        out.push(key.char());
    }

    Ok(out)
}

pub fn part1(path: &Path) -> Result<(), Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let code = decode_on(reader, KEYPAD_ORTHO)?;
    println!("code on ortho keys: {}", code);
    Ok(())
}

pub fn part2(path: &Path) -> Result<(), Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let code = decode_on(reader, KEYPAD_DIAG)?;
    println!("code on diag keys: {}", code);
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("unknown instruction")]
    UnknownInstruction,
    #[error("bad keypad")]
    BadKeypad,
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const EXAMPLE: &str = "ULL\nRRDDD\nLURDL\nUUUUD\n";

    #[test]
    /// From the example:
    ///
    /// ```notrust
    /// ULL
    /// RRDDD
    /// LURDL
    /// UUUUD
    /// ```
    ///
    /// produces "5DB3"
    fn test_decode_diag() {
        let result = decode_on(Cursor::new(EXAMPLE), KEYPAD_DIAG)
            .expect("Decoding failed when it shouldn't");
        assert_eq!(result, "5DB3");
    }

    #[test]
    fn test_decode_ortho() {
        let result = decode_on(Cursor::new(EXAMPLE), KEYPAD_ORTHO)
            .expect("Decoding failed when it shouldn't");
        assert_eq!(result, "1985");
    }
}
