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

use aoclib::parse;
use counter::Counter;
use lazy_static::lazy_static;
use regex::Regex;
use std::{num::ParseIntError, path::Path, str::FromStr};

lazy_static! {
    static ref ROOM_RE: Regex = Regex::new(
        r"(?x)^
        (?P<name>[a-zA-Z\-]+)  # room name
        (?:-)                  # noncapturing hyphen.
        (?P<sector>\d+)        # sector number.
        \[(?P<checksum>[a-zA-Z]{5})\]  # checksum
        $"
    )
    .unwrap();
}

#[derive(Debug, parse_display::Display)]
#[display("{name}-{sector}[{checksum}]")]
struct Room {
    name: String,
    sector: u64,
    checksum: String,
}

impl FromStr for Room {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = ROOM_RE.captures(s).ok_or(Error::NoMatch)?;
        Ok(Room {
            name: captures["name"].to_string(),
            sector: captures["sector"].parse()?,
            checksum: captures["checksum"].to_string(),
        })
    }
}

impl Room {
    /// Construct a checksum per the Santa Rules
    fn make_checksum(&self) -> String {
        let mut counter = Counter::<_, usize>::init(self.name.chars());
        counter.remove(&'-');
        counter
            .most_common_ordered()
            .into_iter()
            .take(5)
            .map(|(ch, _)| ch)
            .collect()
    }

    /// `true` if this room is valid
    fn is_valid(&self) -> bool {
        self.make_checksum() == self.checksum
    }

    /// Decrypt a room code according to Santa Rules
    ///
    /// 1. shift every char by sector number
    /// 2. dashes become spaces
    fn decrypt(&self) -> String {
        shift_str(&self.name, self.sector).replace("-", " ")
    }

    /// Find valid rooms whose decrypted name contains the words "northpole"
    ///
    /// Case-insensitive
    fn has_north_pole(&self) -> bool {
        self.is_valid() && self.decrypt().to_lowercase().contains("northpole")
    }
}

fn shift_char(mut ch: char, shift: u64) -> char {
    if !ch.is_ascii_alphabetic() {
        return ch;
    }
    let upper = ch.is_uppercase();
    ch.make_ascii_lowercase();
    let ch_idx = ch as u8 - b'a';
    let shift_idx = ((ch_idx as u64 + shift) % 26) as u8;
    ch = (shift_idx + b'a') as char;
    if upper {
        ch.make_ascii_uppercase();
    }
    ch
}

/// En/decrypt a string using a shift cypher
pub fn shift_str(encrypted: &str, shift: u64) -> String {
    encrypted.chars().map(|ch| shift_char(ch, shift)).collect()
}

pub fn part1(path: &Path) -> Result<(), Error> {
    let valid_sector_sum: u64 = parse::<Room>(path)?
        .filter(|room| room.is_valid())
        .map(|room| room.sector)
        .sum();
    println!("valid count: {}", valid_sector_sum);
    Ok(())
}

pub fn part2(path: &Path) -> Result<(), Error> {
    println!("rooms with north pole:");
    for room in parse::<Room>(path)?.filter(|room| room.has_north_pole()) {
        println!("  {}", room.sector);
    }
    Ok(())
}

pub fn list_decrypted(path: &Path) -> Result<(), Error> {
    for room in parse::<Room>(path)? {
        println!("{}", room.decrypt());
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("no match for room regex")]
    NoMatch,
    #[error("parsing sector")]
    ParseSector(#[from] ParseIntError),
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLES: &[(&'static str, bool)] = &[
        ("aaaaa-bbb-z-y-x-123[abxyz]", true),
        ("a-b-c-d-e-f-g-h-987[abcde]", true),
        ("not-a-real-room-404[oarel]", true),
        ("totally-real-room-200[decoy]", false),
        ("doesn't match the regex", false),
    ];

    #[test]
    fn test_validate() {
        for (room, expected) in EXAMPLES {
            let validity = room
                .parse::<Room>()
                .map(|room| room.is_valid())
                .unwrap_or_default();
            assert_eq!(validity, *expected);
        }
    }

    #[test]
    fn test_sum_valid_sectors() {
        let sum_valid: u64 = EXAMPLES
            .iter()
            .filter_map(|(room, _)| room.parse::<Room>().ok())
            .filter(|room| room.is_valid())
            .map(|room| room.sector)
            .sum();
        assert_eq!(sum_valid, 1514);
    }

    #[test]
    fn test_decrypt() {
        // this doesn't validate, but that doesn't matter:
        // decryption is orthogonal to validation
        let encrypted = "qzmt-zixmtkozy-ivhz-343[bleah]";
        let room: Room = encrypted.parse().unwrap();
        assert_eq!(room.decrypt(), "very encrypted name");
    }
}
