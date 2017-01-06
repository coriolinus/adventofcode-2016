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

extern crate num_bigint;
use num_bigint::BigUint;

extern crate num_traits;
use num_traits::Zero;
use num_traits::cast::FromPrimitive;

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
    // println!("handle_char({:?}, {})", state, input);
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
                // of course, we haven't yet dealt with the input char at all yet
                // simplest solution is to handle it recursively
                let (new_state, additional_output) = handle_char(Normal, input);
                if let Some(additional) = additional_output {
                    output.push_str(&additional);
                }
                (new_state, Some(output))
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
    // we may not have actually emitted any output, if the last character read was one
    // marked
    match state {
        // if we ended just as the marking ended, we still need to write our output
        State::ReadingMarked(length, count, ref marked) if length == 0 => {
            for _ in 0..count {
                output.push_str(marked);
            }
        }
        // normal state is also fine
        State::Normal => {}
        // anything else is probably an error
        _ => return None,
    }
    Some(output)
}

fn parse_marker<I>(input: &mut I) -> Option<(usize, usize, usize)>
    where I: Iterator<Item = (usize, char)>
{
    let length = input.by_ref()
        .map(|(_, c)| c)
        .take_while(|c| *c != 'x')
        .collect::<String>()
        .parse::<usize>();
    let (index, count): (Vec<usize>, String) =
        input.by_ref().take_while(|&(_, c)| c != ')').unzip();
    // index is last updated on the character preceding the close paren.
    // What we want to return is the index of the close paren
    // Therefore, simply add one.
    let index = *index.last().unwrap() + 1;
    let count = count.parse::<usize>();

    match (length, count) {
        (Ok(length), Ok(count)) => Some((index, length, count)),
        _ => None,
    }
}

/// Return the length of the decompressed data, or None if there's a parse error.
///
/// It is a parse error for a repeated section to end within a marker.
///
/// Computes in a single pass over the data. Let's work through an example,
/// to show how that works:
///
/// ```notrust
/// Input:  X(8x2)(3x3)ABCY
///                   11111
/// Index:  012345678901234
///                    ^^^- Marked by the 3x3 marker immediately preceding
///               ^^^^^^^^- Marked by the 8x2 marker
/// M'cand: 1----------6661
/// ```
///
/// So how does it work? The iterator simply keeps track of the marked sections as it proceeds.
/// The initial character, `X`, has a multiplicand of 1; it's not part of any marked section.
/// The next character, `(`, triggers a `parse_marker` call; those characters are not counted.
/// It returns the tuple `(8, 2)`, corresponding to its settings; at this time, the current index
/// is `6`; this is used to compute that a 2x multiplier should be applied through position `13`.
///
/// The next character is also `(`, which triggers the same sort of computation. The
/// `parse_marker` call consumed through character 10, so the current index is 11. This is
/// used to compute that a 3x multiplier should be applied through position `13`.
///
/// The next three characters, 11-13, have both multipliers applied, for a total multiplicand
/// of 6. Finally, both multipliers expire, so the final character as position 14 is applied once.
pub fn count_decompressed_v2<I>(input: &mut I) -> Option<BigUint>
    where I: Iterator<Item = char>
{
    let mut multipliers: Vec<(usize, usize)> = Vec::new(); // (until, multiplicand)
    let mut multiplicand; // we'll initialize it in the loop, later
    let mut total: BigUint = Zero::zero();

    let mut enumerated = input.enumerate();
    while let Some((index, ch)) = enumerated.next() {
        // first, add all appropriate counts
        multipliers.retain(|&(until, _)| index <= until);

        // if this was an open paren, parse that
        if ch == '(' {
            if let Some((index, length, count)) = parse_marker(&mut enumerated.by_ref()) {
                // println!("Parsed marker as ({}x{}); index {} => until {}",
                //          length,
                //          count,
                //          index,
                //          (index + length));
                multipliers.push((index + length, count));
            } else {
                return None;
            }
            // The following assignment is only ever read if we're emitting debug output,
            // which we aren't anymore, as the program now works. Therefore, we just
            // disable this assignment for efficiency.
            // multiplicand = 0;
        } else {
            multiplicand = multipliers.iter()
                .map(|&(_, multiplicand)| multiplicand as u64)
                .product();
            total = total + BigUint::from_u64(multiplicand).unwrap();
        }

        // println!("{}: '{}' * {} ({:?}) => {}",
        //          index,
        //          ch,
        //          multiplicand,
        //          multipliers,
        //          total);
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate num_bigint;
    use num_bigint::BigUint;

    extern crate num_traits;
    use num_traits::cast::FromPrimitive;

    fn get_examples() -> Vec<&'static str> {
        vec![
            "ADVENT",
            "A(1x5)BC",
            "(3x3)XYZ",
            "A(2x2)BCD(2x2)EFG",
            "(6x1)(1x3)A",
            "X(8x2)(3x3)ABCY",
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

    #[test]
    fn test_count_v2() {
        let expected = vec![
            ("(3x3)XYZ", 9),
            ("X(8x2)(3x3)ABCY", 20),
            ("(27x12)(20x12)(13x14)(7x10)(1x12)A", 241920),
            ("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN", 445),
        ];
        for (case, ex_len) in expected {
            println!("Decompressing: {}", case);
            let length = count_decompressed_v2(&mut case.chars());
            println!("Case '{}' -> Expect '{}', Found {:?}", case, ex_len, length);
            assert!(length == Some(BigUint::from_u64(ex_len).unwrap()));
        }
    }
}
