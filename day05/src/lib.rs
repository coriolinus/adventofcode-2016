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

fn next_suffix_with_valid_hash(prefix: &str, initial_suffix: u64) -> Option<(u64, char, char)> {
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
            let hashresults = hasher.result_str();
            let mut hexdigits = hashresults.chars().skip(5);
            let position = hexdigits.next()
                .expect("md5 should produce more than 5 digits of output");
            let value = hexdigits.next()
                .expect("md5 should produce more than 6 digits of output");
            return Some((suffix, position, value));
        }
        hasher.reset();
    }
    None
}

#[cfg(test)]
/// Hack to enable testing of module private function above
pub fn pub_next_suffix_with_valid_hash(prefix: &str,
                                       initial_suffix: u64)
                                       -> Option<(u64, char, char)> {
    next_suffix_with_valid_hash(prefix, initial_suffix)
}

struct GetPassword {
    prefix: String,
    current_suffix: u64,
}

impl GetPassword {
    fn new(prefix: &str) -> GetPassword {
        GetPassword {
            prefix: prefix.to_string(),
            current_suffix: 0,
        }
    }
}

impl Iterator for GetPassword {
    type Item = (usize, char);

    fn next(&mut self) -> Option<(usize, char)> {
        while let Some((last_suffix, position, value)) =
                  next_suffix_with_valid_hash(&self.prefix, self.current_suffix) {
            self.current_suffix = last_suffix + 1;
            if let Some(position) = match position {
                pos @ '0'...'7' => Some(pos as usize - '0' as usize),
                _ => None,
            } {
                return Some((position, value));
            }
        }
        None
    }
}

pub fn get_password(prefix: &str) -> [char; 8] {
    let mut password: [Option<char>; 8] = [None; 8];
    let mut gp = GetPassword::new(prefix);

    // until all positions are filled
    while !password.iter().all(|position| position.is_some()) {
        if let Some((position, value)) = gp.next() {
            if password[position].is_none() {
                password[position] = Some(value);
            }
        } else {
            panic!("ran out of suffix possibilities!");
        }
    }

    // copy results into output array
    let mut result = [' '; 8];
    for i in 0..8 {
        result[i] = password[i].expect("not all positions contained values");
    }
    result
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
        let expected = Some((should_work, '1'));

        assert!(pub_next_suffix_with_valid_hash(prefix, should_work) == expected);
        assert!(pub_next_suffix_with_valid_hash(prefix, should_work - 1) == expected);
    }

    #[test]
    #[ignore]
    /// Test function which gets next passing number.
    ///
    /// This runs the examples, which are somewhat expensive.
    /// Therefore, ignored by default.
    /// To run the ignored tests:
    /// `cargo test -- --ignored`
    fn test_get_next() {
        let prefix = "abc";
        let result0 = pub_next_suffix_with_valid_hash(prefix, 0);
        println!("First result: {:?}", result0);

        assert!(result0 == Some((3231929, '1')));

        let result1 = pub_next_suffix_with_valid_hash(prefix, 3231930);
        println!("Second result: {:?}", result1);

        assert!(result1 == Some((5017308, '8')));

        let result2 = pub_next_suffix_with_valid_hash(prefix, 5017309);
        println!("Third result: {:?}", result2);

        assert!(result2 == Some((5278568, 'f')));
    }

    #[test]
    #[ignore]
    fn test_get_first_eight() {
        let result = GetPassword::new("abc").take(8).collect::<String>();
        assert!(&result == "18f47a30");
    }
}
