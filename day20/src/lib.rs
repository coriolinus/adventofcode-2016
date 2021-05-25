use aoclib::parse;
use itertools::Itertools;
use std::{
    ops::{Bound, RangeBounds},
    path::Path,
};

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
    let mut rules: Vec<_> = rules.collect();
    debug_assert!(rules.iter().all(|Rule(low, high)| low <= high));
    rules.sort_unstable();
    rules
        .into_iter()
        .coalesce(|Rule(prev_low, prev_high), Rule(next_low, next_high)| {
            // coalesce adjacent blacklist ranges into a combined range with a unified lower, upper
            if next_low <= prev_high {
                Ok(Rule(prev_low, prev_high.max(next_high)))
            } else {
                Err((Rule(prev_low, prev_high), Rule(next_low, next_high)))
            }
        })
}

fn lowest_legal_value(rules: impl Iterator<Item = Rule>) -> Option<u32> {
    let mut iter = ordered_rules_iter_from(rules).peekable();
    if let Some(Rule(low, _)) = iter.peek() {
        if *low > 0 {
            return Some(0);
        }
    }
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
    num_legal_values_in(rules, ..)
}

fn num_legal_values_in(rules: impl Iterator<Item = Rule>, bounds: impl RangeBounds<u32>) -> u32 {
    let lower_bound_inclusive = match bounds.start_bound() {
        Bound::Included(v) => *v,
        Bound::Excluded(v) => *v + 1,
        Bound::Unbounded => 0,
    };
    let upper_bound_inclusive = match bounds.end_bound() {
        Bound::Included(v) => *v,
        Bound::Excluded(v) => *v - 1,
        Bound::Unbounded => u32::MAX,
    };
    let mut count = 0;
    let mut iter = ordered_rules_iter_from(rules).peekable();
    if let Some(Rule(low, _)) = iter.peek() {
        if low.checked_sub(lower_bound_inclusive).unwrap_or_default() > 0 {
            count += low;
        }
    }
    while let Some(Rule(_, prev_high)) = iter.next() {
        count += match iter.peek() {
            None => upper_bound_inclusive - prev_high,
            Some(Rule(next_low, _)) if next_low.checked_sub(prev_high).unwrap_or_default() > 1 => {
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

#[cfg(test)]
mod tests {
    use super::*;
    use aoclib::input::parse_str;

    const EXAMPLE: &str = "5-8
    0-2
    4-7";

    #[test]
    fn test_example_part2() {
        assert_eq!(num_legal_values_in(parse_str(EXAMPLE).unwrap(), 0..10), 2);
    }

    #[test]
    fn test_open_low() {
        let rules = || parse_str("2-9").unwrap();
        assert_eq!(lowest_legal_value(rules()).unwrap(), 0);
        assert_eq!(num_legal_values_in(rules(), 0..10), 2);
    }

    #[test]
    fn test_open_high() {
        let rules = || parse_str("0-7").unwrap();
        assert_eq!(lowest_legal_value(rules()).unwrap(), 8);
        assert_eq!(num_legal_values_in(rules(), 0..10), 2);
    }

    #[test]
    fn test_overlap_1() {
        let rules = || {
            parse_str(
                "0-0
                0-1
                1-2
                2-8",
            )
            .unwrap()
        };
        assert_eq!(lowest_legal_value(rules()).unwrap(), 9);
        assert_eq!(num_legal_values_in(rules(), ..10), 1);
    }

    #[test]
    fn test_overlap_0() {
        let rules = || {
            parse_str(
                "0-0
                1-1
                2-2
                3-8",
            )
            .unwrap()
        };
        assert_eq!(lowest_legal_value(rules()).unwrap(), 9);
        assert_eq!(num_legal_values_in(rules(), ..10), 1);
    }

    #[test]
    fn test_gap_1() {
        let rules = || {
            parse_str(
                "0-0
                2-2
                4-8",
            )
            .unwrap()
        };
        assert_eq!(lowest_legal_value(rules()).unwrap(), 1);
        assert_eq!(num_legal_values_in(rules(), ..10), 3);
    }

    #[test]
    fn test_overlap_2() {
        let rules = || {
            parse_str(
                "0-0
                0-1
                0-2
                1-8",
            )
            .unwrap()
        };
        assert_eq!(lowest_legal_value(rules()).unwrap(), 9);
        assert_eq!(num_legal_values_in(rules(), ..10), 1);
    }

    #[test]
    fn test_range_merge_naive() {
        let rules = || {
            parse_str(
                "0-6
                1-1
                8-9",
            )
            .unwrap()
        };
        assert_eq!(lowest_legal_value(rules()).unwrap(), 7);
        assert_eq!(num_legal_values_in(rules(), ..10), 1);
    }
}
