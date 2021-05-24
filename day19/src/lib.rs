use aoclib::parse;

use std::path::Path;

fn clear_leading_one(mut n: u32) -> u32 {
    if n != 0 {
        let mask = !(1 << (31 - n.leading_zeros()));
        n &= mask;
    }
    n
}

// Oh man! I get the pun in the title ("An Elephant Named Joseph"), because this is the
// Josephus problem, and I did some work on that as an undergrad. I think this is literally
// the first time that an AoC problem has been a re-statement of an obscure math thing,
// and I immediately recognized the obscure math thing. I love it!
fn josephus(n: u32) -> u32 {
    let l = clear_leading_one(n);
    2 * l + 1
}

pub fn part1(input: &Path) -> Result<(), Error> {
    for input in parse(input)? {
        println!("solution for {}: {}", input, josephus(input));
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

    #[test]
    fn test_clear_leading_one() {
        let mut n = !0;
        while n != 0 {
            assert_eq!(n >> 1, clear_leading_one(n));
            n >>= 1;
        }
    }

    #[test]
    fn test_josephus() {
        let expect = [1, 1, 3, 1, 3, 5, 7, 1, 3, 5, 7, 9, 11, 13, 15, 1];
        for (n, expect) in (1..).zip(std::array::IntoIter::new(expect)) {
            assert_eq!(josephus(n), expect);
        }
    }
}
