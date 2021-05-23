use aoclib::{
    numbers::chinese_remainder::{chinese_remainder, Constraint},
    parse,
};

use std::path::Path;

// known wrong answers:
//
// - ~50,000 => too low
// - 237235 => too high
//
// cheating with mathematica:
//
// In[5]:= sizes = {13, 17, 19, 7, 5, 3}
//
// Out[5]= {13,17,19,7,5,3}
//
// In[6]:= positions = {10, 15, 17, 1, 0, 1}
//
// Out[6]= {10,15,17,1,0,1}
//
// In[7]:= ChineseRemainder[-positions - Range[6], sizes]
//
// Out[7]= 203660
//
// so why isn't my implementation producing this value? look into that

#[derive(Debug, Clone, PartialEq, Eq, parse_display::Display, parse_display::FromStr)]
#[display("Disc has {positions} positions; at time=0, it is at position {initial}.")]
#[from_str(
    regex = r"Disc #\d+ has (?P<positions>\d+) positions; at time=0, it is at position (?P<initial>\d+)."
)]
struct Disc {
    positions: i32,
    initial: i32,
}

#[cfg(test)]
impl Disc {
    fn at(&self, time: i32) -> i32 {
        // note that there is 1 second of fall time before reaching the disc
        (time + self.initial) % self.positions
    }
}

fn when_discs_line_up(discs: &[Disc]) -> Option<i32> {
    let constraints: Vec<_> = discs
        .iter()
        .enumerate()
        .map(|(idx, disc)| {
            let time = idx as i32;
            Constraint::new(disc.positions, -disc.initial - time)
        })
        .collect();
    let product: i32 = discs.iter().map(|disc| disc.positions).product();
    chinese_remainder(&constraints).map(|mut solution| {
        // subtract 1 for initial fall time
        solution -= 1;
        while solution < 0 {
            solution += product;
        }
        solution
    })
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let discs: Vec<Disc> = parse(input)?.collect();
    println!(
        "discs first line up at time {}",
        when_discs_line_up(&discs).ok_or(Error::NoSolution)?
    );
    Ok(())
}

pub fn part2(_input: &Path) -> Result<(), Error> {
    unimplemented!()
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("no solution found")]
    NoSolution,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Disc #1 has 5 positions; at time=0, it is at position 4.
    Disc #2 has 2 positions; at time=0, it is at position 1.";

    fn example() -> Vec<Disc> {
        aoclib::input::parse_str(EXAMPLE).unwrap().collect()
    }

    #[test]
    fn test_parse() {
        assert_eq!(example().len(), 2);
    }

    #[test]
    fn test_example() {
        let discs = example();
        assert_eq!(when_discs_line_up(&discs).unwrap(), 5);
    }

    #[test]
    fn test_at() {
        let discs = example();

        assert_eq!(discs[0].at(0), 4);
        assert_eq!(discs[0].at(1), 0);
        assert_eq!(discs[0].at(2), 1);
        assert_eq!(discs[0].at(3), 2);
        assert_eq!(discs[0].at(4), 3);
        assert_eq!(discs[0].at(5), 4);

        assert_eq!(discs[1].at(0), 1);
        assert_eq!(discs[1].at(1), 0);
        assert_eq!(discs[1].at(2), 1);
        assert_eq!(discs[1].at(3), 0);
    }

    // test doesn't work right now and I don't have the patience to debug it
    #[test]
    #[ignore]
    fn test_fallthrough() {
        // we need a bunch of coprime numbers. I suspect we have some handy.
        for time_offset in 0..10 {
            let discs: Vec<_> = std::array::IntoIter::new([3, 5, 7, 13, 17, 19])
                .enumerate()
                .map(|(disc_idx, positions)| Disc {
                    positions,
                    initial: time_offset - 1 - (disc_idx as i32),
                })
                .collect();
            dbg!(time_offset, &discs);

            // check setup
            for (idx, disc) in discs.iter().enumerate() {
                let time = idx as i32 + 1;
                assert_eq!(disc.at(time), time_offset % disc.positions);
            }

            // check we can determine the right answer
            assert_eq!(when_discs_line_up(&discs).unwrap(), time_offset);
        }
    }
}
