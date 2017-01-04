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

pub fn make_checksum(string: &str) -> String {
    unimplemented!()
}
