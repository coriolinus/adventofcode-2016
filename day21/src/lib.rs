use aoclib::parse;

use std::{collections::VecDeque, path::Path};

const INPUT_PART1: &str = "abcdefgh";
const INPUT_PART2: &str = "fbgdceah";

#[derive(Debug, Clone, Copy, PartialEq, Eq, parse_display::Display, parse_display::FromStr)]
#[display(style = "lowercase")]
enum Direction {
    Left,
    Right,
}

/// Compute the absolute position that a `RotateOn` character should end up at to reverse a
/// `RotateOn` application.
///
/// This function was derived by analysis of the forward rotation transform for positions `0..8`.
fn reverse_rotate_position(mut pos: usize) -> usize {
    if pos == 0 {
        return 7;
    }
    if pos % 2 == 0 {
        pos += 8;
    }
    pos -= 1;
    pos / 2
}

/// Compute the left rotation that should be used to unapply a `RotateOn` transform.
fn reverse_rotate(pos: usize) -> usize {
    (8 + pos - reverse_rotate_position(pos)) % 8
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, parse_display::Display, parse_display::FromStr)]
enum Operation {
    #[display("swap position {0} with position {1}")]
    SwapPosition(usize, usize),
    #[display("swap letter {0} with letter {1}")]
    SwapLetter(char, char),
    #[display("rotate {0} {1} steps")]
    #[from_str(regex = r"rotate (?P<0>\w+) (?P<1>\d+) steps?")]
    Rotate(Direction, usize),
    #[display("rotate based on position of letter {0}")]
    RotateOn(char),
    #[display("reverse positions {0} through {1}")]
    Reverse(usize, usize),
    #[display("move position {0} to position {1}")]
    Move(usize, usize),
}

impl Operation {
    fn apply(self, buffer: &mut VecDeque<u8>) {
        match self {
            Self::SwapPosition(a, b) => buffer.swap(a, b),
            Self::SwapLetter(a, b) => {
                let (a, b) = (a as u8, b as u8);
                buffer.iter_mut().for_each(|c| {
                    if *c == a {
                        *c = b;
                    } else if *c == b {
                        *c = a;
                    }
                })
            }
            Self::Rotate(direction, by) => match direction {
                Direction::Left => buffer.rotate_left(by),
                Direction::Right => buffer.rotate_right(by),
            },
            Self::RotateOn(c) => {
                let c = c as u8;
                let pos = buffer.iter().position(|ch| *ch == c).expect(&format!(
                    "attemped to rotate on '{}' but that char not in buffer",
                    c as char
                ));
                let rot = 1 + pos + if pos >= 4 { 1 } else { 0 };
                buffer.rotate_right(rot);
            }
            Self::Reverse(a, b) => {
                buffer.make_contiguous()[a..=b].reverse();
            }
            Self::Move(from, to) => {
                let c = buffer.remove(from).expect("attempted to remove from an idx out of range");
                buffer.insert(to, c);
            }
        }
    }

    fn unapply(self, buffer: &mut VecDeque<u8>) {
        match self {
            Self::SwapPosition(a, b) => buffer.swap(a, b),
            Self::SwapLetter(a, b) => {
                let (a, b) = (a as u8, b as u8);
                buffer.iter_mut().for_each(|c| {
                    if *c == a {
                        *c = b;
                    } else if *c == b {
                        *c = a;
                    }
                })
            }
            Self::Rotate(direction, by) => match direction {
                Direction::Left => buffer.rotate_right(by),
                Direction::Right => buffer.rotate_left(by),
            },
            Self::RotateOn(c) => {
                let c = c as u8;
                let pos = buffer.iter().position(|ch| *ch == c).expect(&format!(
                    "attemped to rotate on '{}' but that char not in buffer",
                    c as char
                ));
                // reversing the position can probably be done more elegantly, but this should work:
                let rot = reverse_rotate(pos);
                buffer.rotate_left(rot);
            }
            Self::Reverse(a, b) => {
                buffer.make_contiguous()[a..=b].reverse();
            }
            Self::Move(to, from) => {
                let c = buffer.remove(from).expect("attempted to remove from an idx out of range");
                buffer.insert(to, c);
            }
        }
    }
}

fn scramble(input: &str, operations: impl Iterator<Item = Operation>) -> String {
    let mut buffer: VecDeque<u8> = input.as_bytes().iter().copied().collect();
    for operation in operations {
        operation.apply(&mut buffer);
    }
    String::from_utf8(buffer.into_iter().collect()).expect("scramble operations shouldn't remove utf8-ness")
}

fn unscramble(input: &str, operations: impl Iterator<Item = Operation>) -> String {
    // we have to reverse the operations, and we don't have a DoubleEndedIterator, so...
    let mut operations: Vec<_> = operations.collect();
    operations.reverse();

    let mut buffer: VecDeque<u8> = input.as_bytes().iter().copied().collect();
    for operation in operations {
        operation.unapply(&mut buffer);
    }
    String::from_utf8(buffer.into_iter().collect()).expect("scramble operations shouldn't remove utf8-ness")
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let scrambled = scramble(INPUT_PART1, parse(input)?);
    println!("scrambled password: {}", scrambled);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let unscrambled = unscramble(INPUT_PART2, parse(input)?);
    println!("scrambled password: {}", unscrambled);
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
