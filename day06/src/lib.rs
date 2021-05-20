//! Advent of Code - Day 06 Instructions
//!
//! Signals and Noise
//!
//! Something is jamming your communications with Santa. Fortunately, your signal is only
//! partially jammed, and protocol in situations like this is to switch to a simple repetition
//! code to get the message through.
//!
//! In this model, the same message is sent repeatedly. You've recorded the repeating message
//! signal (your puzzle input), but the data seems quite corrupted - almost too badly to recover.
//! Almost.
//!
//! All you need to do is figure out which character is most frequent for each position.
//! For example, suppose you had recorded the following messages:
//!
//! ```notrust
//! eedadn
//! drvtee
//! eandsr
//! raavrd
//! atevrs
//! tsrnev
//! sdttsa
//! rasrtv
//! nssdts
//! ntnada
//! svetve
//! tesnvt
//! vntsnd
//! vrdear
//! dvrsen
//! enarar
//! ```
//!
//! The most common character in the first column is e; in the second, a; in the third, s,
//! and so on. Combining these characters returns the error-corrected message, `easter`.
//!
//! Given the recording in your puzzle input, what is the error-corrected version of
//! the message being sent?

use aoclib::parse;
use counter::Counter;
use std::path::Path;

/// Compose a string in which each character is the most or least common from the input lines.
///
/// Every input line must have an equal number of characters for this to work right.
fn count_frequent<Lines>(lines: Lines, want_greatest: bool) -> Option<String>
where
    Lines: IntoIterator<Item = String>,
    <Lines as IntoIterator>::IntoIter: Clone,
{
    let iter = lines.into_iter();
    let width = iter.clone().next()?.len();

    let mut output = String::with_capacity(width);
    for idx in 0..width {
        let counter: Counter<u8> = iter.clone().map(|str| str.as_bytes()[idx]).collect();
        let ordering = counter.most_common_ordered();
        let (superlative, _) = if want_greatest {
            ordering.first()?
        } else {
            ordering.last()?
        };
        output.push(*superlative as char);
    }

    Some(output)
}

/// Compose a string in which each character is the most common from the input lines.
///
/// Every input line must have an equal number of characters for this to work right.
fn count_most_frequent<Lines>(lines: Lines) -> Option<String>
where
    Lines: IntoIterator<Item = String>,
    <Lines as IntoIterator>::IntoIter: Clone,
{
    count_frequent(lines, true)
}

/// Compose a string in which each character is the least common from the input lines.
///
/// Every input line must have an equal number of characters for this to work right.
fn count_least_frequent<Lines>(lines: Lines) -> Option<String>
where
    Lines: IntoIterator<Item = String>,
    <Lines as IntoIterator>::IntoIter: Clone,
{
    count_frequent(lines, false)
}

pub fn part1(path: &Path) -> Result<(), Error> {
    let signals: Vec<String> = parse(path)?.collect();
    if signals.iter().any(|signal| !signal.is_ascii()) {
        return Err(Error::NotAscii);
    }
    let message = count_most_frequent(signals).ok_or(Error::SuperlativeProblem)?;
    println!("message (most frequent): {}", message);
    Ok(())
}

pub fn part2(path: &Path) -> Result<(), Error> {
    let signals: Vec<String> = parse(path)?.collect();
    if signals.iter().any(|signal| !signal.is_ascii()) {
        return Err(Error::NotAscii);
    }
    let message = count_least_frequent(signals).ok_or(Error::SuperlativeProblem)?;
    println!("message (least frequent): {}", message);
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("not all characters were ascii")]
    NotAscii,
    #[error("problem creating a word from the superlative frequencies of the input")]
    SuperlativeProblem,
}
