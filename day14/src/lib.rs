//! Make some _very bad_ one-time pads.
//!
//! Note: part2 is slow, consider testing in release mode.

use aoclib::parse;
use crypto::{digest::Digest, md5::Md5};
use std::{
    collections::VecDeque,
    ops::{Index, IndexMut},
    path::Path,
};

/// `State` keeps track of potential keys.
///
/// A key is added to the potential keys at a certain position
/// if we encounter 3 of the same character in a row.
///
/// It is removed in two cases:
///
/// - If 5 of the same character are in a row, then all entries in the potential keys
///   for that character are validated.
/// - For each index N that we check, remove all keys for which `key + 1000 < N`
// for efficiency, we just keep a separate vector of indices for each possible character
// the indices are the indices at which a triple was discovered.
#[derive(Default)]
struct State([VecDeque<usize>; 16]);

impl State {
    fn idx_for(index: char) -> usize {
        match index {
            _ if ('0'..='9').contains(&index) => (index as u8 - b'0') as usize,
            _ if ('a'..='f').contains(&index) => (index as u8 - b'a') as usize + 10,
            _ => panic!("invalid index for PotentialKeys: {}", index),
        }
    }
}

impl Index<char> for State {
    type Output = VecDeque<usize>;

    fn index(&self, index: char) -> &Self::Output {
        let index = Self::idx_for(index);
        &self.0[index]
    }
}

impl IndexMut<char> for State {
    fn index_mut(&mut self, index: char) -> &mut Self::Output {
        let index = Self::idx_for(index);
        &mut self.0[index]
    }
}

impl State {
    /// Update the state from the hashes at a given index.
    ///
    /// Returns `(insert, key)`, where `key` is a character in the one-time pad
    /// and `insert` is the index at which it was originally inserted as a member
    /// of a triplet.
    fn update(
        &mut self,
        idx: usize,
        triplet: Option<char>,
        quintuplets: impl Iterator<Item = char>,
    ) -> Vec<(usize, char)> {
        // first, clear all pending potential keys which have expired
        // a potential key is expired when its activaction index was
        // more than 1000 ago
        for queue in self.0.iter_mut() {
            while queue
                .front()
                .map(|&insert_idx| idx - insert_idx > 1000)
                .unwrap_or_default()
            {
                queue.pop_front();
            }
        }

        // now compute the return value: fives which have been satisfied
        // by a prior three. Note that we have to compute this _before_ we add
        // the new potential keys from the threes, to avoid the situation in which
        // a lone five without a prior three activates itself.
        let (min_bound, _) = quintuplets.size_hint();
        let mut activated_keys = Vec::with_capacity(min_bound);
        for activated_key in quintuplets {
            activated_keys.extend(
                self[activated_key]
                    .drain(..)
                    .map(|insert_idx| (insert_idx, activated_key)),
            );
        }

        // finally add the new potential key to the tracked state
        if let Some(potential_key) = triplet {
            // note that we have to deduplicate
            if !self[potential_key]
                .back()
                .map(|&last_idx| last_idx == idx)
                .unwrap_or_default()
            {
                self[potential_key].push_back(idx);
            }
        }

        activated_keys
    }
}

/// make a function which, given an integer, computes its salted hash
fn make_hash_for(salt: &str) -> impl Fn(usize) -> String {
    let mut digest = Md5::new();
    digest.input_str(salt);
    move |idx| {
        let mut digest = digest.clone();
        digest.input_str(&idx.to_string());
        digest.result_str()
    }
}

/// make a function which, given an integer, computes its stretched, salted hash
fn make_stretched_hash_for(salt: &str) -> impl Fn(usize) -> String {
    let mut digest = Md5::new();
    digest.input_str(salt);

    move |idx| {
        let mut digest = digest.clone();
        digest.input_str(&idx.to_string());

        for _ in 0..2016 {
            let hash = digest.result_str();
            digest.reset();
            digest.input_str(&hash);
        }

        digest.result_str()
    }
}

// important! only consider the first triplet in any given hash
fn first_triplet_in(hash: &str) -> Option<char> {
    hash.as_bytes()
        .windows(3)
        .filter(|window| window[0] == window[1] && window[1] == window[2])
        .map(|window| window[0] as char)
        .next()
}

fn quintuplets_in(hash: &str) -> impl '_ + Iterator<Item = char> {
    hash.as_bytes()
        .windows(5)
        .filter(|window| {
            window
                .windows(2)
                .all(|subwindow| subwindow[0] == subwindow[1])
        })
        .map(|window| window[0] as char)
}

/// Generate a onetime pad using the specified hash-maker.
///
/// Return the pad and the index which produced its 64th character.
fn generate_onetime_pad(make_hash: impl Fn(usize) -> String) -> (String, usize) {
    let mut state = State::default();
    let mut keys = Vec::with_capacity(64);

    let mut idx = 0;
    while keys.len() < 64 {
        let hash = make_hash(idx);
        keys.extend(state.update(idx, first_triplet_in(&hash), quintuplets_in(&hash)));
        idx += 1;
    }

    keys.truncate(64);
    keys.sort_unstable();
    let (final_insert, _) = keys.last().unwrap().clone();
    let pad = keys.into_iter().map(|(_, key)| key).collect();

    (pad, final_insert)
}

pub fn part1(input: &Path, show_pad: bool) -> Result<(), Error> {
    for salt in parse::<String>(input)? {
        let (pad, idx) = generate_onetime_pad(make_hash_for(&salt));
        println!("salt {}: generates at idx {}", salt, idx);
        if show_pad {
            println!("  pad: {}", pad);
        }
    }
    Ok(())
}

pub fn part2(input: &Path, show_pad: bool) -> Result<(), Error> {
    for salt in parse::<String>(input)? {
        let (pad, idx) = generate_onetime_pad(make_stretched_hash_for(&salt));
        println!("salt {}: generates (stretched) at idx {}", salt, idx);
        if show_pad {
            println!("  pad: {}", pad);
        }
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Make a function which returns true if the input iterator contains a particular character of interest.
    ///
    /// The generated function's signature is `Fn(impl Iterator<Item=char>) -> bool`.
    ///
    /// Has to be a macro because rust doesn't support returning a function which is itself generic
    /// unless the generic parameters are known at call time of the outer call.
    macro_rules! make_has_char {
        (fn $name:ident() for $want:expr) => {
            fn $name(repeated_digits: impl IntoIterator<Item = char>) -> bool {
                repeated_digits
                    .into_iter()
                    .find_map(|ch| (ch == $want).then(|| ()))
                    .is_some()
            }
        };
    }

    /// demonstrate that we get the right results from partial construction
    ///
    /// partial construction is important for us becasue it lets us hash the
    /// salt _once_ instead of for every iteration.
    #[test]
    fn test_partial_construction() {
        let rust_crypto_md5 = {
            let mut digest = Md5::new();
            digest.input_str("abc");
            digest.input_str("123");
            digest.result_str()
        };

        let md5_crate_md5 = md5::compute("abc123");

        assert_eq!(rust_crypto_md5, format!("{:x}", md5_crate_md5));
    }

    #[test]
    fn example_eights() {
        let hash_for = make_hash_for("abc");
        make_has_char!(fn has_eight() for '8');

        for idx in 0..18 {
            let hash = hash_for(idx);
            assert!(!has_eight(first_triplet_in(&hash)));
        }

        let hash = hash_for(18);
        assert!(has_eight(first_triplet_in(&hash)));

        for idx in 19..=1018 {
            let hash = hash_for(idx);
            assert!(!has_eight(quintuplets_in(&hash)));
        }
    }

    #[test]
    fn example_es() {
        let hash_for = make_hash_for("abc");
        make_has_char!(fn has_e() for 'e');

        for idx in 0..39 {
            let hash = hash_for(idx);
            assert!(!has_e(first_triplet_in(&hash)));
        }

        let hash = hash_for(39);
        assert!(has_e(first_triplet_in(&hash)));

        for idx in 40..816 {
            let hash = hash_for(idx);
            assert!(!has_e(quintuplets_in(&hash)));
        }

        let hash = hash_for(816);
        assert!(has_e(quintuplets_in(&hash)));
    }

    #[test]
    fn stretched_hash_example() {
        let stretched_hash_for = make_stretched_hash_for("abc");
        assert_eq!(stretched_hash_for(0), "a107ff634856bb300138cac6568c0f24");
    }

    #[test]
    fn stretched_example_2s() {
        let stretched_hash_for = make_stretched_hash_for("abc");
        make_has_char!(fn has_2() for '2');

        for idx in 0..5 {
            let hash = stretched_hash_for(idx);
            assert!(first_triplet_in(&hash).is_none());
        }

        let hash = stretched_hash_for(5);
        assert!(has_2(first_triplet_in(&hash)));

        for idx in 6..=1005 {
            let hash = stretched_hash_for(idx);
            assert!(!has_2(quintuplets_in(&hash)));
        }
    }

    #[test]
    fn stretched_example_es() {
        let stretched_hash_for = make_stretched_hash_for("abc");
        make_has_char!(fn has_e() for 'e');

        for idx in 0..10 {
            let hash = stretched_hash_for(idx);
            assert!(!has_e(first_triplet_in(&hash)));
        }

        let hash = stretched_hash_for(10);
        assert!(has_e(first_triplet_in(&hash)));

        for idx in 11..89 {
            let hash = stretched_hash_for(idx);
            assert!(!has_e(quintuplets_in(&hash)));
        }

        let hash = stretched_hash_for(89);
        assert!(has_e(quintuplets_in(&hash)));
    }

    #[test]
    fn full_example() {
        let (pad, idx) = generate_onetime_pad(make_hash_for("abc"));
        dbg!(pad);
        assert_eq!(idx, 22728);
    }

    #[test]
    fn full_stretched_example() {
        let (pad, idx) = generate_onetime_pad(make_stretched_hash_for("abc"));
        dbg!(pad);
        assert_eq!(idx, 22551);
    }
}
