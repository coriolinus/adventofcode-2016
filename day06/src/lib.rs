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

extern crate counter;
use counter::Counter;

/// Transpose a vector of vectors into new memory
///
/// May panic or truncate data if input data is not rectangular.
pub fn transpose<T: Copy + Default>(input: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let old_rows = input.len();
    if old_rows == 0 {
        return Vec::new();
    }
    let old_cols = input[0].len();

    // initialize new memory
    let mut output = vec![vec![T::default(); old_rows]; old_cols];

    // copy data
    for i in 0..old_rows {
        for j in 0..old_cols {
            output[j][i] = input[i][j];
        }
    }

    output
}

/// Compose a string in which each character is the most common from the input lines.
///
/// Every input line must have an equal number of characters for this to work right.
pub fn count_most_frequent(lines: &str) -> String {
    let input = lines.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let transposed = transpose(&input);

    let mut output = String::with_capacity(transposed.len());

    for chars in transposed {
        output.push(
            Counter::init(chars).most_common().next().expect("At least one row required!").0
        );
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_transpose_example() -> Vec<Vec<usize>> {
        vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ]
    }

    #[test]
    fn test_transpose_simple() {
        let expected_output = vec![
            vec![1, 4],
            vec![2, 5],
            vec![3, 6],
        ];

        assert!(transpose(&get_transpose_example()) == expected_output);
    }

    #[test]
    fn test_double_transpose_is_noop() {
        assert!(transpose(&transpose(&get_transpose_example())) == get_transpose_example());
    }

    fn get_lines_example() -> String {
        "\
eedadn\n\
drvtee\n\
eandsr\n\
raavrd\n\
atevrs\n\
tsrnev\n\
sdttsa\n\
rasrtv\n\
nssdts\n\
ntnada\n\
svetve\n\
tesnvt\n\
vntsnd\n\
vrdear\n\
dvrsen\n\
enarar\n\
        "
            .to_string()
    }

    #[test]
    fn test_given_example() {
        assert!(&count_most_frequent(&get_lines_example()) == "easter");
    }
}
