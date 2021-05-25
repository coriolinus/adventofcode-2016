use aoclib::parse;

use std::{
    fmt,
    ops::{Deref, DerefMut},
    path::Path,
    str::FromStr,
};

// known wrong:
// - 10100011110100011 is too high

#[derive(Debug, Clone)]
struct Data(Vec<bool>);

impl FromStr for Data {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes = Vec::with_capacity(s.len());
        for ch in s.chars() {
            match ch {
                '0' => bytes.push(false),
                '1' => bytes.push(true),
                _ => return Err(Error::UnexpectedChar(ch)),
            }
        }

        Ok(Data(bytes))
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for bit in self.0.iter() {
            write!(f, "{}", if *bit { 1 } else { 0 })?;
        }
        Ok(())
    }
}

impl Deref for Data {
    type Target = Vec<bool>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Data {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Data {
    fn dragon(&self) -> Self {
        let mut next = self.clone();
        next.reserve(1 + self.len());
        next.push(false);
        for bit in self.iter().rev() {
            next.push(!*bit);
        }
        next
    }

    fn dragon_fill(&self, want_bits: usize) -> Self {
        let mut dragon = self.clone();
        while dragon.len() < want_bits {
            dragon = dragon.dragon();
        }
        dragon.truncate(want_bits);
        dragon
    }

    fn checksum(&self) -> Self {
        let mut data = self.0.clone();
        let mut next = Vec::with_capacity(data.len());

        while data.len() % 2 == 0 {
            next.clear();
            for pair in data.chunks(2) {
                next.push(pair[0] == pair[1]);
            }
            std::mem::swap(&mut data, &mut next);
        }

        Self(data)
    }
}

const PART1_SIZE: usize = 272;
const PART2_SIZE: usize = 35651584;

pub fn part1(input: &Path) -> Result<(), Error> {
    for initial_state in parse::<Data>(input)? {
        let filled = initial_state.dragon_fill(PART1_SIZE);
        let checksum = filled.checksum();
        println!(
            "Given {}, size {}, checksum is {}",
            initial_state, PART1_SIZE, checksum
        );
    }
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    for initial_state in parse::<Data>(input)? {
        let filled = initial_state.dragon_fill(PART2_SIZE);
        let checksum = filled.checksum();
        println!(
            "Given {}, size {}, checksum is {}",
            initial_state, PART2_SIZE, checksum
        );
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("unexpected char '{0}'")]
    UnexpectedChar(char),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        for (init, want) in std::array::IntoIter::new([
            ("1", "100"),
            ("0", "001"),
            ("11111", "11111000000"),
            ("111100001010", "1111000010100101011110000"),
        ]) {
            let data = Data::from_str(init).unwrap();
            let data = data.dragon();
            assert_eq!(data.to_string(), want);
        }
    }
}
