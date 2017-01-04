//! Advent of Code - Day 04 Instructions
//!
//! Security through Obscurity
//!
//! Finally, you come across an information kiosk with a list of rooms. Of course, the list is
//! encrypted and full of decoy data, but the instructions to decode the list are barely hidden
//! nearby. Better remove the decoy data first.
//!
//! Each room consists of an encrypted name (lowercase letters separated by dashes) followed by
//! a dash, a sector ID, and a checksum in square brackets.
//!
//! A room is real (not a decoy) if the checksum is the five most common letters in the encrypted
//! name, in order, with ties broken by alphabetization. For example:
//!
//! - `aaaaa-bbb-z-y-x-123[abxyz]` is a real room because the most common letters are a (5),
//!    b (3), and then a tie between x, y, and z, which are listed alphabetically.
//! - `a-b-c-d-e-f-g-h-987[abcde]` is a real room because although the letters are all tied
//!    (1 of each), the first five are listed alphabetically.
//! - `not-a-real-room-404[oarel]` is a real room.
//! - `totally-real-room-200[decoy]` is not.
//!
//! Of the real rooms from the list above, the sum of their sector IDs is 1514.
//!
//! What is the sum of the sector IDs of the real rooms?


#[macro_use]
extern crate lazy_static;

// this was _way_ more work than this problem strictly required, but then that's
// 200 or so lines of code nobody else has to write in Rust again.
extern crate counter;
use counter::Counter;

// Clearly we're going to have to parse these strings, and it looks like regexes are the
// right tool for the job. In python, I'd use this one:
// r"^(?P<name>[a-zA-Z\-]+)(?:(?<!\-)-)(?P<sector>\d+)\[(?P<checksum>[a-zA-Z]{5})\]$"
// We just need to translate that to Rust.
extern crate regex;
use regex::Regex;

lazy_static! {
    static ref ROOM_RE: Regex = Regex::new(r"(?x)^
        (?P<name>[a-zA-Z\-]+)  # room name
        (?:-)                  # noncapturing hyphen.
        (?P<sector>\d+)        # sector number.
        \[(?P<checksum>[a-zA-Z]{5})\]  # checksum
        $").unwrap();
}

/// Construct a checksum per the Santa Rules
pub fn make_checksum(string: &str) -> String {
    let mut counter = Counter::init(string.chars());
    counter.map.remove(&'-');
    counter.most_common_ordered().take(5).map(|(ch, _)| ch).collect()
}

/// `true` if a room string is valid
pub fn validate(room: &str) -> bool {
    if let Some(captures) = ROOM_RE.captures(room) {
        make_checksum(&captures["name"]) == captures["checksum"]
    } else {
        false
    }
}

pub fn count_valid_lines(lines: &str) -> usize {
    lines.lines().map(|line| line.trim()).filter(|line| validate(line)).count()
}

pub fn sum_valid_sectors(lines: &str) -> usize {
    lines.lines()
        .map(|line| line.trim())
        .filter(|line| validate(line))
        .map(|line| {
            ROOM_RE.captures(line).unwrap()["sector"]
                .parse::<usize>()
                .expect("Error parsing sector ID as usize")
        })
        .sum()
}


fn shift_char(mut ch: char, shift: usize) -> char {
    use std::ascii::AsciiExt;

    let upper = ch.is_uppercase();
    ch.make_ascii_lowercase();
    const ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";
    if let Some(index) = ALPHABET.find(ch) {
        ch = ALPHABET.chars().nth((index + shift) % 26).unwrap();
    }
    if upper {
        ch.make_ascii_uppercase();
    }
    ch
}

/// En/decrypt a string using a shift cypher
pub fn shift_str(encrypted: &str, shift: usize) -> String {
    encrypted.chars().map(|ch| shift_char(ch, shift)).collect()
}

/// Decrypt a room code according to Santa Rules
///
/// 1. shift every char by sector number
/// 2. dashes become spaces
pub fn decrypt(encrypted: &str) -> Option<String> {
    if let Some(captures) = ROOM_RE.captures(encrypted) {
        let shift = captures["sector"].parse::<usize>().expect("Failed to parse sector as usize");
        let decrypted = shift_str(&captures["name"], shift);
        Some(decrypted.replace("-", " "))
    } else {
        None
    }
}

/// Find valid rooms whose decrypted name contains the words "north pole"
///
/// Case-insensitive
pub fn find_np_lines(lines: &str) -> Vec<(String, usize)> {
    lines.lines()
        .map(|line| line.trim())
        .filter(|line| validate(line))
        .map(|line| {
            let caps = ROOM_RE.captures(line).unwrap();
            (decrypt(&line).unwrap(),
             caps["sector"].parse::<usize>().expect("Failed to parse sector as usize"))
        })
        .filter(|&(ref name, _)| name.to_lowercase().contains("north pole"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_examples() -> Vec<String> {
        vec![
            "aaaaa-bbb-z-y-x-123[abxyz]",
            "a-b-c-d-e-f-g-h-987[abcde]",
            "not-a-real-room-404[oarel]",
            "totally-real-room-200[decoy]",
            "doesn't match the regex",
        ]
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn test_validate() {
        for (room, expected) in get_examples()
            .iter()
            .zip([true, true, true, false, false].into_iter()) {
            assert!(validate(room) == *expected);
        }
    }

    #[test]
    fn test_count_valid_lines() {
        let lines = get_examples().join("\n");
        assert!(count_valid_lines(&lines) == 3);
    }

    #[test]
    fn test_sum_valid_sectors() {
        let lines = get_examples().join("\n");
        assert!(sum_valid_sectors(&lines) == 1514);
    }

    #[test]
    fn test_decrypt() {
        // this doesn't validate, but that doesn't matter:
        // decryption is orthogonal to validation
        let encrypted = "qzmt-zixmtkozy-ivhz-343[bleah]";
        assert!(&decrypt(encrypted).expect("failed to decrypt test string") ==
                "very encrypted name");
    }
}
