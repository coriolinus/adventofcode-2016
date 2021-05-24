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

fn ordered_rules_iter_from(rules: impl Iterator<Item = Rule>) -> impl Iterator<Item = Rule> {
    let rules: BTreeSet<_> = rules
        .chain(std::iter::once(Rule::default()))
        .inspect(|Rule(low, high)| debug_assert!(low <= high))
        .collect();
    rules.into_iter()
}

fn lowest_legal_value(rules: impl Iterator<Item = Rule>) -> Option<u32> {
    let mut iter = ordered_rules_iter_from(rules).peekable();
    while let Some(Rule(_, prev_high)) = iter.next() {
        match iter.peek() {
            None if prev_high < u32::MAX - 1 => return Some(prev_high + 1),
            Some(Rule(next_low, _)) if *next_low > prev_high + 1 => return Some(prev_high + 1),
            _ => {}
        }
    }

    None
}

fn num_legal_values(rules: impl Iterator<Item = Rule>) -> u32 {
    let mut count = 0;
    let mut iter = ordered_rules_iter_from(rules).peekable();
    while let Some(Rule(_, prev_high)) = iter.next() {
        count += match iter.peek() {
            None => u32::MAX - prev_high,
            Some(Rule(next_low, _)) if prev_high < u32::MAX - 1 && *next_low > prev_high + 1 => {
                next_low - prev_high - 1
            }
            _ => 0,
        }
    }

    count
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let llv = lowest_legal_value(parse(input)?).ok_or(Error::NoSolution)?;
    println!("lowest legal value: {}", llv);
    Ok(())
}

// known wrong: too high: 801988815
pub fn part2(input: &Path) -> Result<(), Error> {
    let legal_values = num_legal_values(parse(input)?);
    println!("num legal values: {}", legal_values);
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("no value is legal")]
    NoSolution,
}
