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

use aoclib::parse;
use crypto::digest::Digest;
use crypto::md5::Md5;
use std::{borrow::Borrow, path::Path};

#[cfg(feature = "parallelism")]
use rayon::prelude::*;

fn make_hash_for(prefix: &str) -> impl '_ + Fn(u64) -> (u64, String) {
    let key = prefix.as_bytes();
    move |suffix| {
        let mut hasher = Md5::new();
        hasher.input(key);
        hasher.input(suffix.to_string().as_bytes());
        (suffix, hasher.result_str())
    }
}

/// If the first five characters of `hash` are all `0`, returns the characters at index 5 and 6
/// if both are set.
fn zero_five_six(tuple: impl Borrow<(u64, String)>) -> Option<(u64, char, char)> {
    let (suffix, hash) = tuple.borrow();

    let mut five = None;
    let mut six = None;
    for (idx, ch) in hash.chars().enumerate().take(7) {
        match idx {
            _ if idx < 5 && ch != '0' => return None,
            5 => five = Some(ch),
            6 => six = Some(ch),
            _ => {}
        }
    }

    match (five, six) {
        (Some(five), Some(six)) => Some((*suffix, five, six)),
        _ => None,
    }
}

/// Return the tuple `(suffix, five, six)`.
#[cfg(feature = "parallelism")]
fn next_valid_suffix(prefix: &str, initial_suffix: u64) -> Option<(u64, char, char)> {
    let hash_for = make_hash_for(prefix);
    (initial_suffix..=u64::MAX)
        .into_par_iter()
        .map(hash_for)
        .find_map_first(zero_five_six)
}

/// Return the tuple `(suffix, five, six)`.
#[cfg(not(feature = "parallelism"))]
fn next_valid_suffix(prefix: &str, initial_suffix: u64) -> Option<(u64, char, char)> {
    let hash_for = make_hash_for(prefix);
    (initial_suffix..=u64::MAX)
        .map(hash_for)
        .find_map(zero_five_six)
}

struct SuffixIter<'a>(&'a str, u64);

impl<'a> SuffixIter<'a> {
    fn new(prefix: &'a str) -> SuffixIter<'a> {
        SuffixIter(prefix, 0)
    }
}

impl<'a> Iterator for SuffixIter<'a> {
    type Item = (char, char);

    fn next(&mut self) -> Option<Self::Item> {
        let (suffix, five, six) = next_valid_suffix(self.0, self.1)?;
        self.1 = suffix + 1;
        Some((five, six))
    }
}

fn make_password_simple(prefix: &str) -> Option<String> {
    let mut password = String::with_capacity(8);
    password.extend(SuffixIter::new(prefix).take(8).map(|(five, _)| five));
    (password.len() == 8).then(move || password)
}

fn make_password_fancy(prefix: &str) -> Option<String> {
    let mut password = vec![None; 8];
    let mut iter = SuffixIter::new(prefix);
    while password.iter().any(|maybe_char| maybe_char.is_none()) {
        let (five, six) = iter.next()?;
        let idx = match (five as u8).checked_sub(b'0') {
            Some(idx) if idx < 8 => idx as usize,
            _ => continue,
        };
        if password[idx].is_none() {
            password[idx] = Some(six);
        }
    }
    password.into_iter().collect()
}

pub fn part1(path: &Path) -> Result<(), Error> {
    for door_input in parse::<String>(path)? {
        let password =
            make_password_simple(&door_input).ok_or_else(|| Error::NotFound(door_input.clone()))?;
        println!("simple password for {}: {}", door_input, password);
    }
    Ok(())
}

pub fn part2(path: &Path) -> Result<(), Error> {
    for door_input in parse::<String>(path)? {
        let password =
            make_password_fancy(&door_input).ok_or_else(|| Error::NotFound(door_input.clone()))?;
        println!("fancy password for {}: {}", door_input, password);
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("could not determine a password for \"{0}\"")]
    NotFound(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test get_next using a known input which should return immediately,
    /// and one which should iterate once before success.
    ///
    /// This ensure the function works, without being as expensive as a full zero-knowledge run.
    fn test_get_next_works() {
        let prefix = "abc";
        let should_work = 3231929;

        assert!(matches!(
            next_valid_suffix(prefix, should_work),
            Some((suffix, '1', _)) if suffix == should_work,
        ));
        assert!(matches!(
            next_valid_suffix(prefix, should_work - 1),
            Some((suffix, '1', _)) if suffix == should_work,
        ));
    }

    #[test]
    /// Test function which gets next passing number.
    fn test_get_next() {
        let prefix = "abc";
        let result = next_valid_suffix(prefix, 0);
        assert!(matches!(result, Some((3231929, '1', _))));

        let result = next_valid_suffix(prefix, 3231930);
        assert!(matches!(result, Some((5017308, '8', _))));

        let result = next_valid_suffix(prefix, 5017309);
        assert!(matches!(result, Some((5278568, 'f', _))));
    }

    #[test]
    fn test_get_first_eight() {
        let result = make_password_simple("abc").unwrap();
        assert_eq!(result, "18f47a30");
    }

    #[test]
    fn test_suffix_iter() {
        let mut iter = SuffixIter::new("abc");

        assert_eq!(iter.next(), Some(('1', '5')));
        assert_eq!(iter.next(), Some(('8', 'f')));
        assert_eq!(iter.next(), Some(('f', '9')));
        assert_eq!(iter.next(), Some(('4', 'e')));
    }

    #[test]
    fn test_password_fancy() {
        assert_eq!(make_password_fancy("abc").unwrap(), "05ace8e3");
    }
}
