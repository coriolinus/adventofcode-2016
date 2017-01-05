//! Advent of Code - Day 07 Instructions
//!
//! Internet Protocol Version 7
//!
//! While snooping around the local network of EBHQ, you compile a list of IP addresses
//! (they're IPv7, of course; IPv6 is much too limited). You'd like to figure out which IPs
//! support TLS (transport-layer snooping).
//!
//! An IP supports TLS if it has an Autonomous Bridge Bypass Annotation, or ABBA. An ABBA
//! is any four-character sequence which consists of a pair of two different characters
//! followed by the reverse of that pair, such as xyyx or abba. However, the IP also must
//! not have an ABBA within any hypernet sequences, which are contained by square brackets.
//!
//! For example:
//!
//! - `abba[mnop]qrst` supports TLS (abba outside square brackets).
//! - `abcd[bddb]xyyx` does not support TLS (bddb is within square brackets, even though xyyx
//!    is outside square brackets).
//! - `aaaa[qwer]tyui` does not support TLS (aaaa is invalid; the interior characters
//!    must be different).
//! - `ioxxoj[asdfgh]zxcvbn` supports TLS (oxxo is outside square brackets, even though it's
//!    within a larger string).
//!
//! How many IPs in your puzzle input support TLS?

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Day07Err {
    UnmatchedBrackets,
    NestedBrackets(usize, usize),
    ReversedBrackets(usize, usize),
}

/// Split a string into a list of substrings, split by square brackets.
///
/// Return the section and whether or not it occurs within brackets.
///
/// Nested or unmatched brackets cause this to return an error.
pub fn split_brackets<'a>(input: &'a str) -> Result<Vec<(&'a str, bool)>, Day07Err> {
    // ensure we have the same number of brackets
    if input.chars().filter(|&c| c == '[').count() != input.chars().filter(|&c| c == ']').count() {
        return Err(Day07Err::UnmatchedBrackets);
    }

    // otherwise, match them into sections, and check those
    let open_brackets = input.match_indices('[').map(|t| t.0);
    let close_brackets = input.match_indices(']').map(|t| t.0);
    let bracket_sections = open_brackets.zip(close_brackets).collect::<Vec<_>>();

    // validate that we have sane brackets
    for &(start, end) in bracket_sections.iter() {
        if start >= end {
            return Err(Day07Err::ReversedBrackets(start, end));
        }
    }
    for window in bracket_sections.windows(2) {
        let (start1, end1) = window[0];
        let (start2, _) = window[1];

        if end1 >= start2 {
            return Err(Day07Err::NestedBrackets(start1, start2));
        }
    }

    let mut result = Vec::new();
    let mut index = 0;

    // for each bracketed section, we append two sections:
    // those elements before the opening bracket,
    // and those within
    //
    // then, we append a section containing everything after the final bracket
    //
    // Example: the string `abba[mnop]qrst`
    // will map start, end once, at (4, 9)
    // we create three substrings: [[0..4], [5..9], [10..14]]
    for (start, end) in bracket_sections {
        if start > index {
            // true if the bracket wasn't the first character
            result.push((&input[index..start], false));
        }
        if (end - start) > 1 {
            // true if there are characters between the brackets
            result.push((&input[(start + 1)..end], true))
        }
        index = end + 1;
    }
    result.push((&input[index..], false));

    Ok(result)
}

pub fn contains_abba(input: &str) -> bool {
    if input.len() < 4 {
        return false;
    }

    // we can't just use the Vec::windows(4) implementation, which does precisely what we want,
    // if only this were a vec instead of a string.
    // Instead, we have to do something like that ourselves.
    //
    // Example: "abracadabra"
    //           ^   ^- ends = 4. example[0..4] == "abra"
    //           |----- starts = 0.
    //
    let starts = input.char_indices().map(|(i, _)| i);
    let ends = input.char_indices().map(|(i, _)| i).skip(4);

    // to avoid reallocating everything as a vector of chars,
    // we have to look at it as bytes instead. This of course
    // means that we're vulnerable to errors if we encounter some unicode,
    // but we _shouldn't_ encounter that for this problem.
    let bytes = input.as_bytes();

    for (start, _) in starts.zip(ends) {
        if bytes[start] != bytes[start + 1] && // first two don't match
            bytes[start + 1] == bytes[start + 2] && // inner two match
            bytes[start] == bytes[start + 3]
        // outer two match
        {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_examples() -> Vec<String> {
        vec![
            "abba[mnop]qrst",
            "abcd[bddb]xyyx",
            "aaaa[qwer]tyui",
            "ioxxoj[asdfgh]zxcvbn",
        ]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn test_split_brackets_happy() {
        let expected = vec![
            vec![
                ("abba", false),
                ("mnop", true),
                ("qrst", false),
            ],
            vec![
                ("abcd", false),
                ("bddb", true),
                ("xyyx", false),
            ],
            vec![
                ("aaaa", false),
                ("qwer", true),
                ("tyui", false),
            ],
            vec![
                ("ioxxoj", false),
                ("asdfgh", true),
                ("zxcvbn", false),
            ],
        ];

        for (example, expect) in get_examples().iter().zip(expected) {
            assert!(split_brackets(&example) == Ok(expect));
        }
    }

    #[test]
    fn test_split_brackets_unmatched() {
        for case in ["[", "]", "[][", "][]"].iter() {
            assert!(split_brackets(case) == Err(Day07Err::UnmatchedBrackets));
        }
    }

    #[test]
    fn test_split_brackets_nested() {
        for case in ["[[]]", "[][[]]", "[[[]]]"].iter() {
            match split_brackets(case) {
                Err(Day07Err::NestedBrackets(_, _)) => {}
                e => {
                    println!("{:?}", e);
                    panic!("Wrong error returned");
                }
            }
        }
    }

    #[test]
    fn test_split_brackets_reversed() {
        for case in ["][", "[]][", "[][]][", "][[]"].iter() {
            match split_brackets(case) {
                Err(Day07Err::ReversedBrackets(_, _)) => {}
                e => {
                    println!("{:?}", e);
                    panic!("Wrong error returned");
                }
            }
        }
    }

    #[test]
    fn test_contains_abba() {
        for (case, expect) in get_examples().iter().zip([true, true, false, true].into_iter()) {
            assert!(contains_abba(case) == *expect);
        }
    }
}
