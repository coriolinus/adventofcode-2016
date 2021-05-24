use aoclib::parse;

use std::{cell::Cell, collections::VecDeque, path::Path, rc::Rc};

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
    (clear_leading_one(n) << 1) | 1
}

pub fn part1(input: &Path) -> Result<(), Error> {
    for input in parse(input)? {
        println!("solution for {}: {}", input, josephus(input));
    }
    Ok(())
}

// This is pretty bad: `O(n**2)`: VecDeque rotation requires `O(n)`
fn josephus_across(n: u32) -> u32 {
    let mut players: VecDeque<_> = (1..=n).collect();
    while players.len() > 1 {
        let shift = players.len() / 2;
        players.rotate_left(shift);
        players.pop_front();
        players.rotate_right(shift - 1);
    }
    players[0]
}

// getting a solution still requires `O(n)`, but that's acceptable, where
// the naive implementation isn't.
fn josephus_across_from_iter(n: u32) -> u32 {
    josephus_across_iter()
        .nth((n - 1) as usize)
        .expect("josephus_across_iter never terminates")
}

fn partial_josephus_across_iter(n: u32) -> impl Iterator<Item = u32> {
    (1..=n).chain((1..=n).map(move |m| 2 * m + n))
}

fn josephus_across_iter() -> impl Iterator<Item = u32> {
    let mut sub_iter: Box<dyn Iterator<Item = u32>> = Box::new(partial_josephus_across_iter(3));

    // this is a bit ugly, but it's forced on us: we're creating two references
    // to a single `Cell`, which gives us interior mutability. That means that
    // we can hand one of them to the `from_fn` closure, which can read it, and
    // another to the `inspect` closure, which can update it.
    //
    // We could work around this by implementing Iterator manually on a struct, but
    // I wanted to do this the lazy way instead.
    let prev1 = Rc::new(Cell::new(0));
    let prev2 = prev1.clone();

    std::array::IntoIter::new([1, 1, 3]).chain(
        std::iter::from_fn(move || {
            Some(match sub_iter.next() {
                Some(v) => v,
                None => {
                    // set the sub-iterator, but skip its first value; we know that it always
                    // equals 1
                    sub_iter = Box::new(partial_josephus_across_iter(prev1.get()).skip(1));
                    1
                }
            })
        })
        .inspect(move |v| {
            prev2.set(*v);
        }),
    )
}

// oh well, I was hoping this would be super simple, but I guess I can actually implement
// this problem.
pub fn part2(input: &Path) -> Result<(), Error> {
    for input in parse(input)? {
        println!(
            "solution across for {}: {}",
            input,
            josephus_across_from_iter(input)
        );
    }
    Ok(())
}

pub fn first_100_across() {
    for n in 1..=100 {
        println!("josephus_across({}) -> {}", n, josephus_across(n));
    }
    println!("resets:");
    print!("1");
    for n in 2..=100 {
        if josephus_across(n) == 1 {
            print!(", {}", n);
        }
    }
    println!();
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

    #[test]
    fn test_josephus_across_example() {
        assert_eq!(josephus_across(5), 2);
    }

    #[test]
    fn test_partial_josephus_across_iter() {
        let expect = [
            (3, vec![1, 2, 3, 5, 7, 9]),
            (
                9,
                vec![
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27,
                ],
            ),
        ];
        for (n, expect) in expect.iter() {
            assert!(partial_josephus_across_iter(*n)
                .zip(expect)
                .all(|(have, want)| have == *want));
        }
    }

    #[test]
    fn test_josephus_across_iter() {
        assert!((1..=100)
            .zip(josephus_across_iter())
            .all(|(n, have)| josephus_across(n) == have));
    }

    #[test]
    fn test_josephus_across_from_iter() {
        for n in 1..=100 {
            assert_eq!(josephus_across(n), josephus_across_from_iter(n));
        }
    }
}
