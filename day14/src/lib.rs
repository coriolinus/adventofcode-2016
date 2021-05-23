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
    /// Returns keys which should be added to the one-time pad.
    fn update(
        &mut self,
        idx: usize,
        threes: impl Iterator<Item = char>,
        fives: impl Iterator<Item = char>,
    ) -> impl '_ + Iterator<Item = char> {
        // first, clear all pending potential keys which have expired
        // a potential key is expired when its activaction index was
        // more than 1000 ago
        for queue in self.0.iter_mut() {
            while queue
                .front()
                .map(|&item_idx| idx - item_idx > 1000)
                .unwrap_or_default()
            {
                queue.pop_front();
            }
        }

        // now compute the return value: fives which have been satisfied
        // by a prior three. Note that we have to compute this _before_ we add
        // the new potential keys from the threes, to avoid the situation in which
        // a lone five without a prior three activates itself.
        let (min_bound, _) = fives.size_hint();
        let mut activated_keys = Vec::with_capacity(min_bound);
        for activated_key in fives {
            activated_keys.extend(
                self[activated_key]
                    .drain(..)
                    .map(|idx| (idx, activated_key)),
            );
        }
        activated_keys.sort_unstable();
        // we'll get weird, hard-to-debug errors if this condition ever proves false:
        // the keys are sorted by their insertion index but also alphabetically, so if
        // two keys ever get inserted and activated on the same block, we have at least a
        // 50% chance of emitting them in the wrong order.
        //
        // that does seem unlikely, which is why this is just asserted instead of engineering
        // around the problem, but it's worth the runtime cost of the check.
        assert!(
            activated_keys
                .windows(2)
                .all(|window| window[0].0 != window[1].0),
            "duplicate activated key indices",
        );

        // finally add new potential keys to the tracked state
        for potential_key in threes {
            // note that we have to deduplicate
            if !self[potential_key]
                .back()
                .map(|&last_idx| last_idx == idx)
                .unwrap_or_default()
            {
                self[potential_key].push_back(idx);
            }
        }

        activated_keys.into_iter().map(|(_idx, key)| key)
    }
}

/// make a function which, given an integer, computes its salted hash
fn make_hash_for(salt: &str) -> impl Fn(usize) -> String {
    let mut digest = Md5::new();
    digest.input_str(&salt);
    move |idx| {
        let mut digest = digest.clone();
        digest.input_str(&idx.to_string());
        digest.result_str()
    }
}

fn matches_three(hash: &str) -> impl '_ + Iterator<Item = char> {
    hash.as_bytes()
        .windows(3)
        .filter(|window| window[0] == window[1] && window[1] == window[2])
        .map(|window| window[0] as char)
}

fn matches_five(hash: &str) -> impl '_ + Iterator<Item = char> {
    hash.as_bytes()
        .windows(5)
        .filter(|window| {
            window
                .windows(2)
                .all(|subwindow| subwindow[0] == subwindow[1])
        })
        .map(|window| window[0] as char)
}

pub fn part1(input: &Path) -> Result<(), Error> {
    for salt in parse::<String>(input)? {
        let make_hash = make_hash_for(&salt);

        let mut state = State::default();
        let mut pad = String::with_capacity(64);

        let mut idx = 0;
        while pad.len() < 64 {
            let hash = make_hash(idx);
            pad.extend(state.update(idx, matches_three(&hash), matches_five(&hash)));
            idx += 1;
        }

        println!("for salt {}, idx {} produces the 64th key", salt, idx - 1);
    }
    Ok(())
}

pub fn part2(_input: &Path) -> Result<(), Error> {
    unimplemented!()
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
            fn $name(mut repeated_digits: impl Iterator<Item = char>) -> bool {
                repeated_digits
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
            assert!(!has_eight(matches_three(&hash)));
        }

        let hash = hash_for(18);
        assert!(has_eight(matches_three(&hash)));

        for idx in 19..=1018 {
            let hash = hash_for(idx);
            assert!(!has_eight(matches_five(&hash)));
        }
    }

    #[test]
    fn example_es() {
        let hash_for = make_hash_for("abc");
        make_has_char!(fn has_e() for 'e');

        for idx in 0..39 {
            let hash = hash_for(idx);
            assert!(!has_e(matches_three(&hash)));
        }

        let hash = hash_for(39);
        assert!(has_e(matches_three(&hash)));

        for idx in 40..816 {
            let hash = hash_for(idx);
            assert!(!has_e(matches_five(&hash)));
        }

        let hash = hash_for(816);
        assert!(has_e(matches_five(&hash)));
    }
}
