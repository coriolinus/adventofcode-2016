use aoclib::parse;

use std::{collections::BTreeSet, path::Path};

#[derive(
    Default,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    parse_display::Display,
    parse_display::FromStr,
)]
#[display("{0}-{1}")]
struct Rule(u32, u32);

// not right answer: too low: 1888889
fn lowest_legal_value(rules: impl Iterator<Item = Rule>) -> Option<u32> {
    let rules: BTreeSet<_> = rules
        .chain(std::iter::once(Rule::default()))
        .inspect(|Rule(low, high)| debug_assert!(low <= high))
        .collect();
    let mut iter = rules.into_iter().peekable();
    while let Some(Rule(_, prev_high)) = iter.next() {
        match iter.peek() {
            None if prev_high < u32::MAX - 1 => return Some(prev_high + 1),
            Some(Rule(next_low, _)) if *next_low > prev_high + 1 => return Some(prev_high + 1),
            _ => {}
        }
    }

    None
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let llv = lowest_legal_value(parse(input)?).ok_or(Error::NoSolution)?;
    println!("lowest legal value: {}", llv);
    Ok(())
}

pub fn part2(_input: &Path) -> Result<(), Error> {
    unimplemented!()
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("no value is legal")]
    NoSolution,
}
