//! Advent of Code - Day 05 Instructions
//!
//! How about a nice game of chess?
//!
//! You are faced with a security door designed by Easter Bunny engineers that seem to have
//! acquired most of their security knowledge by watching hacking movies.
//!
//! The eight-character password for the door is generated one character at a time by finding
//! the MD5 hash of some Door ID (your puzzle input) and an increasing integer index
//! (starting with 0).
//!
//! A hash indicates the next character in the password if its hexadecimal representation
//! starts with five zeroes. If it does, the sixth character in the hash is the next
//! character of the password.
//!
//! For example, if the Door ID is abc:
//!
//! The first index which produces a hash that starts with five zeroes is 3231929, which we
//! find by hashing abc3231929; the sixth character of the hash, and thus the first character
//! of the password, is 1.
//!
//! 5017308 produces the next interesting hash, which starts with 000008f82..., so the second
//! character of the password is 8.
//!
//! The third time a hash starts with five zeroes is for abc5278568, discovering the
//! character f.
//!
//! In this example, after continuing this search a total of eight times, the password is 18f47a30.
//!
//! Given the actual Door ID, what is the password?

extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn next_suffix_with_valid_hash(prefix: &str, initial_suffix: u64) -> Option<(u64, char)> {
    let mut hasher = Md5::new();

    let key = prefix.as_bytes();
    for suffix in initial_suffix..std::u64::MAX {
        hasher.input(key);
        hasher.input(suffix.to_string().as_bytes());

        let mut output = [0; 16]; // An MD5 is 16 bytes
        hasher.result(&mut output);

        // count first five as digits instead of rendering as hex
        let first_five = output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32;
        if first_five == 0 {
            let sixth_hex = hasher.result_str()
                .chars()
                .skip(5)
                .next()
                .expect("md5 should produce more than 5 digits of output");
            return Some((suffix, sixth_hex));
        }
        hasher.reset();
    }
    None
}

pub struct GetPassword {
    prefix: String,
    current_suffix: u64,
}

impl GetPassword {
    pub fn new(prefix: &str) -> GetPassword {
        GetPassword {
            prefix: prefix.to_string(),
            current_suffix: 0,
        }
    }
}

impl Iterator for GetPassword {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if let Some((next_suffix, ch)) = next_suffix_with_valid_hash(&self.prefix,
                                                                     self.current_suffix) {
            self.current_suffix = next_suffix;
            Some(ch)
        } else {
            None
        }
    }
}
