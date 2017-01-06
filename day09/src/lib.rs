//! Advent of Code - Day 09 Instructions
//!
//! Explosives in Cyberspac3
//!
//! Wandering around a secure area, you come across a datalink port to a new part of the network.
//! After briefly scanning it for interesting files, you find one file in particular that catches
//! your attention. It's compressed with an experimental format, but fortunately, the
//! documentation for the format is nearby.
//!
//! The format compresses a sequence of characters. Whitespace is ignored. To indicate that some
//! sequence should be repeated, a marker is added to the file, like (10x2). To decompress this
//! marker, take the subsequent 10 characters and repeat them 2 times. Then, continue reading the
//! file after the repeated data. The marker itself is not included in the decompressed output.
//!
//! If parentheses or other characters appear within the data referenced by a marker, that's
//! okay - treat it like normal data, not a marker, and then resume looking for markers after
//! the decompressed section.
//!
//! For example:
//!
//! - `ADVENT` contains no markers and decompresses to itself with no changes, resulting in a
//!    decompressed length of 6.
//! - `A(1x5)BC` repeats only the B a total of 5 times, becoming `ABBBBBC` for a decompressed
//!    length of 7.
//! - `(3x3)XYZ` becomes `XYZXYZXYZ` for a decompressed length of 9.
//! - `A(2x2)BCD(2x2)EFG` doubles the BC and EF, becoming `ABCBCDEFEFG` for a decompressed
//!    length of 11.
//! - `(6x1)(1x3)A` simply becomes `(1x3)A` - the `(1x3)` looks like a marker, but because
//!    it's within a data section of another marker, it is not treated any differently from
//!    the A that comes after it. It has a decompressed length of 6.
//! - `X(8x2)(3x3)ABCY` becomes `X(3x3)ABC(3x3)ABCY` (for a decompressed length of 18), because
//!    the decompressed data from the (8x2) marker (the (3x3)ABC) is skipped and not processed
//!    further.
//!
//! What is the decompressed length of the file (your puzzle input)? Don't count whitespace.


#[derive(Debug, PartialEq, Eq)]
enum State {
    Normal,
    ParsingMarkerLength(String), // store WIP characters
    ParsingMarkerCount(usize, String), // store the length, WIP characters
    ReadingMarked(usize, usize, String), // store length, count of repetitions, WIP chars
    Error(&'static str), // error message
}

impl Default for State {
    fn default() -> State {
        State::Normal
    }
}

// Given an input character and the current state, return the subsequent state
// and optionally an output character to write.
fn handle_char(state: State, input: char) -> (State, String) {
    use State::*;
    match state {
        Normal => {
            if input != '(' {
                (Normal, input.to_string())
            } else {
                (ParsingMarkerLength(String::new()), String::new())
            }
        }
        ParsingMarkerLength(mut wip) => {
            if input != 'x' {
                wip.push(input);
                (ParsingMarkerLength(wip), String::new())
            } else {
                if let Ok(length) = wip.parse::<usize>() {
                    (ParsingMarkerCount(length, String::new()), String::new())
                } else {
                    (Error("Could not parse marker length"), String::new())
                }
            }
        }
        ParsingMarkerCount(length, mut wip) => {
            if input != ')' {
                wip.push(input);
                (ParsingMarkerCount(length, wip), String::new())
            } else {
                if let Ok(count) = wip.parse::<usize>() {
                    (ReadingMarked(length, count, String::new()), String::new())
                } else {
                    (Error("Could not parse marker count"), String::new())
                }
            }
        }
        ReadingMarked(mut length, count, mut wip) => {
            if length > 0 {
                wip.push(input);
                length -= 1;
                (ReadingMarked(length, count, wip), String::new())
            } else {
                let mut output = String::with_capacity(count * wip.len());
                for _ in 0..count {
                    output.push_str(&wip);
                }
                (Normal, output)
            }
        }
        error @ Error(_) => (error, String::new()),
    }
}
