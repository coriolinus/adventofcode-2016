//! Advent of Code - Day 01 Instructions
//!
//! You're airdropped near Easter Bunny Headquarters in a city somewhere. "Near", unfortunately,
//! is as close as you can get - the instructions on the Easter Bunny Recruiting Document the
//! Elves intercepted start here, and nobody had time to work them out further.
//!
//! The Document indicates that you should start at the given coordinates (where you just landed)
//! and face North. Then, follow the provided sequence: either turn left (L) or right (R) 90
//! degrees, then walk forward the given number of blocks, ending at a new intersection.
//!
//! There's no time to follow such ridiculous instructions on foot, though, so you take a moment
//! and work out the destination. Given that you can only walk on the street grid of the city,
//! how far is the shortest path to the destination?
//!
//! For example:
//!
//! Following R2, L3 leaves you 2 blocks East and 3 blocks North, or 5 blocks away.
//! R2, R2, R2 leaves you 2 blocks due South of your starting position, which is 2 blocks away.
//! R5, L5, R5, R3 leaves you 12 blocks away.
//! How many blocks away is Easter Bunny HQ?

#[derive(Debug)]
pub enum Rotation {
    Left,
    Right,
}

pub type Directions = Vec<(Rotation, usize)>;

pub fn parse(input: &str) -> Directions {
    let mut results = Vec::with_capacity(input.len() / 4);

    for token in input.split(", ") {
        let (dir_char, dist) = token.split_at(1);

        results.push((match dir_char {
            "L" => Rotation::Left,
            "R" => Rotation::Right,
            _ => panic!("Invalid input; invalid rotation char"),
        },
                      usize::from_str_radix(dist, 10).expect("Invalid input; unparseable distance")));
    }

    results
}
