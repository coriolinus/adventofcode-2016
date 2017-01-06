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

impl State {
    /// Return the error message if this is an error state, or None otherwise
    pub fn error(&self) -> Option<&'static str> {
        match *self {
            State::Error(msg) => Some(msg),
            _ => None,
        }
    }
}

impl Default for State {
    fn default() -> State {
        State::Normal
    }
}

// Given an input character and the current state, return the subsequent state
// and optionally an output character to write.
fn handle_char(state: State, input: char) -> (State, Option<String>) {
    use State::*;
    match state {
        Normal => {
            if input != '(' {
                (Normal, Some(input.to_string()))
            } else {
                (ParsingMarkerLength(String::new()), None)
            }
        }
        ParsingMarkerLength(mut wip) => {
            if input != 'x' {
                wip.push(input);
                (ParsingMarkerLength(wip), None)
            } else {
                if let Ok(length) = wip.parse::<usize>() {
                    (ParsingMarkerCount(length, String::new()), None)
                } else {
                    (Error("Could not parse marker length"), Some(input.to_string()))
                }
            }
        }
        ParsingMarkerCount(length, mut wip) => {
            if input != ')' {
                wip.push(input);
                (ParsingMarkerCount(length, wip), None)
            } else {
                if let Ok(count) = wip.parse::<usize>() {
                    (ReadingMarked(length, count, String::new()), None)
                } else {
                    (Error("Could not parse marker count"), None)
                }
            }
        }
        ReadingMarked(mut length, count, mut wip) => {
            if length > 0 {
                wip.push(input);
                length -= 1;
                (ReadingMarked(length, count, wip), None)
            } else {
                let mut output = String::with_capacity(count * wip.len());
                for _ in 0..count {
                    output.push_str(&wip);
                }
                (Normal, Some(output))
            }
        }
        error @ Error(_) => (error, Some(input.to_string())),
    }
}


/// Decompress the given input according to Santa Rules
pub fn decompress(input: &str) -> Option<String> {
    let mut state = State::default();
    let mut output = String::with_capacity(input.len());

    for ch in input.chars() {
        let result_tuple = handle_char(state, ch);
        state = result_tuple.0;


        if let Some(errmsg) = state.error() {
            println!("Error while decompressing: {}", errmsg);
            println!("Buffer: {}", output);
            return None;
        }

        if let Some(intermediate) = result_tuple.1 {
            output.push_str(&intermediate);
        }
    }
    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_examples() -> Vec<&'static str> {
        vec![
            "ADVENT",
            "A(1x5)BC",
            "(3x3)XYZ",
            "A(2x2)BCD(2x2)EFG",
            "(6x1)(1x3)A",
            "X(8x2)(3x3)ABCY ",
        ]
    }

    #[test]
    fn test_decompress() {
        let expected = vec![
            "ADVENT",
            "ABBBBBC",
            "XYZXYZXYZ",
            "ABCBCDEFEFG",
            "(1x3)A",
            "X(3x3)ABC(3x3)ABCY",
        ];

        for (case, expect) in get_examples().iter().zip(expected.iter().map(|s| s.to_string())) {
            let decompressed = decompress(case);
            println!("Case '{}' -> Expect '{}', Found {:?}",
                     case,
                     expect,
                     decompressed);
            assert!(decompressed == Some(expect));
        }
    }
}
