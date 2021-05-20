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

use aoclib::parse;
use std::path::Path;

/// Assert that this let pattern is irrefutable.
macro_rules! assert_irrefutable {
    (let [ $( $binding:ident ),* $(,)? ] = $e:expr ) => {
        let [ $($binding),* ] = if let [ $($binding),* ] = $e {
            [ $($binding),* ]
        } else {
            unreachable!()
        };
    };
}

/// Split a string into a list of substrings, split by square brackets.
///
/// Return a list of `(section, is_hypernet)`.
///
/// Nested or unmatched brackets cause this to return an error.
pub fn split_brackets(input: &str) -> Result<Vec<(&str, bool)>, Error> {
    // ensure we have the same number of brackets
    if input.chars().filter(|&c| c == '[').count() != input.chars().filter(|&c| c == ']').count() {
        return Err(Error::UnmatchedBrackets);
    }

    // otherwise, match them into sections, and check those
    let open_brackets = input.match_indices('[').map(|t| t.0);
    let close_brackets = input.match_indices(']').map(|t| t.0);
    let bracket_sections = open_brackets.zip(close_brackets).collect::<Vec<_>>();

    // validate that we have sane brackets
    for &(start, end) in bracket_sections.iter() {
        if start >= end {
            return Err(Error::ReversedBrackets(input[end..=start].into()));
        }
    }
    for window in bracket_sections.windows(2) {
        let (start1, end1) = window[0];
        let (start2, _) = window[1];

        if end1 >= start2 {
            return Err(Error::NestedBrackets(input[start1..=start2].into()));
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
    // will map `(start, end)` once, at `(4, 9)`
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

    // to avoid reallocating everything as a vector of chars,
    // we have to look at it as bytes instead. This of course
    // means that we're vulnerable to errors if we encounter some unicode,
    // but we _shouldn't_ encounter that for this problem.
    let bytes = input.as_bytes();

    bytes.windows(4).any(|window| {
        assert_irrefutable!(let [a1, b1, b2, a2] = window);
        a1 != b1 && a1 == a2 && b1 == b2
    })
}

pub fn supports_tls(ipv7: &str) -> bool {
    split_brackets(ipv7)
        .map(|brackets| {
            brackets
                .iter()
                .any(|&(section, is_hypernet)| !is_hypernet && contains_abba(section))
                && !brackets
                    .iter()
                    .any(|&(section, is_hypernet)| is_hypernet && contains_abba(section))
        })
        .unwrap_or_default()
}

/// Compute a list of all ABAs in the contained string.
///
/// Return (a, b)
pub fn contained_abas(input: &str) -> Vec<&str> {
    let mut abas = Vec::new();
    let bytes = input.as_bytes();

    for (start, window) in bytes.windows(3).enumerate() {
        assert_irrefutable!(let [a1, b, a2] = window);
        if a1 != b && a1 == a2 {
            abas.push(&input[start..start + 3]);
        }
    }

    abas
}

/// True if any sequence bab appears in input, given the list of (a, b) pairs
///
/// It's an `O(n**2)` search, but the list of abas should be pretty short.
pub fn contains_bab(input: &str, abas: &[&str]) -> bool {
    abas.iter().any(|aba| {
        assert_irrefutable!(let [a1, b, _a2] = aba.as_bytes());
        let bab_array = [*b, *a1, *b];
        let bab = match std::str::from_utf8(&bab_array) {
            Ok(bab) => bab,
            _ => return false,
        };
        input.contains(bab)
    })
}

pub fn supports_ssl(ipv7: &str) -> bool {
    split_brackets(ipv7)
        .map(|brackets| {
            let (hypernets, supernets): (Vec<_>, Vec<_>) = brackets
                .into_iter()
                .partition(|&(_s, is_hypernet)| is_hypernet);
            let mut abas: Vec<_> = supernets
                .into_iter()
                .flat_map(|(supernet, _)| contained_abas(supernet))
                .collect();
            abas.sort_unstable();
            abas.dedup();

            hypernets
                .into_iter()
                .any(|(hypernet, _)| contains_bab(hypernet, &abas))
        })
        .unwrap_or_default()
}

pub fn part1(path: &Path) -> Result<(), Error> {
    let supports_tls = parse::<String>(path)?
        .filter(|ipv7| supports_tls(ipv7))
        .count();
    println!("supports tls: {}", supports_tls);
    Ok(())
}

pub fn part2(path: &Path) -> Result<(), Error> {
    let supports_ssl = parse::<String>(path)?
        .filter(|ipv7| supports_ssl(ipv7))
        .count();
    println!("supports ssl: {}", supports_ssl);
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("unmatched brackets")]
    UnmatchedBrackets,
    #[error("nested brackets: \"{0}\"")]
    NestedBrackets(String),
    #[error("reversed brackets: \"{0}\"")]
    ReversedBrackets(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLES: &[&str] = &[
        "abba[mnop]qrst",
        "abcd[bddb]xyyx",
        "aaaa[qwer]tyui",
        "ioxxoj[asdfgh]zxcvbn",
    ];

    #[test]
    fn test_split_brackets_happy() {
        let expected = vec![
            vec![("abba", false), ("mnop", true), ("qrst", false)],
            vec![("abcd", false), ("bddb", true), ("xyyx", false)],
            vec![("aaaa", false), ("qwer", true), ("tyui", false)],
            vec![("ioxxoj", false), ("asdfgh", true), ("zxcvbn", false)],
        ];

        for (example, expect) in EXAMPLES.iter().zip(expected) {
            assert!(split_brackets(&example).unwrap() == expect);
        }
    }

    #[test]
    fn test_split_brackets_unmatched() {
        for case in ["[", "]", "[][", "][]"].iter() {
            assert!(matches!(
                split_brackets(case).unwrap_err(),
                Error::UnmatchedBrackets
            ));
        }
    }

    #[test]
    fn test_split_brackets_nested() {
        for case in ["[[]]", "[][[]]", "[[[]]]"].iter() {
            assert!(matches!(
                split_brackets(case),
                Err(Error::NestedBrackets(_))
            ));
        }
    }

    #[test]
    fn test_split_brackets_reversed() {
        for case in ["][", "[]][", "[][]][", "][[]"].iter() {
            assert!(matches!(
                split_brackets(case),
                Err(Error::ReversedBrackets(_))
            ));
        }
    }

    #[test]
    fn test_contains_abba() {
        for (case, expect) in EXAMPLES.iter().zip([true, true, false, true].iter()) {
            assert!(contains_abba(case) == *expect);
        }
        assert!(contains_abba("abba") == true);
        assert!(contains_abba("abb") == false);
        assert!(contains_abba("bccb") == true);
        assert!(contains_abba("aaaa") == false);
    }

    #[test]
    fn test_supports_tls() {
        for (case, expect) in EXAMPLES.iter().zip([true, false, false, true].iter()) {
            println!(
                "Case '{}': expect {} found {}",
                case,
                expect,
                supports_tls(case)
            );
            assert_eq!(supports_tls(case), *expect);
        }
    }

    #[test]
    fn test_supports_ssl() {
        let cases = vec![
            ("aba[bab]xyz", true),
            ("xyx[xyx]xyx", false),
            ("aaa[kek]eke", true),
            ("zazbz[bzb]cdb", true),
        ];
        for (case, expect) in cases {
            println!(
                "Case '{}': expect {} found {}",
                case,
                expect,
                supports_ssl(case)
            );
            assert_eq!(supports_ssl(case), expect);
        }
    }
}
