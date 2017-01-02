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

#[derive(Debug, Copy, Clone)]
pub enum Rotation {
    Left,
    Right,
}

pub type Direction = (Rotation, usize);
pub type Directions = Vec<Direction>;

pub fn parse(input: &str) -> Directions {
    let mut results = Vec::with_capacity(input.len() / 4);

    for token in input.split(", ") {
        let (dir_char, dist) = token.split_at(1);

        let direction = match dir_char {
            "L" => Rotation::Left,
            "R" => Rotation::Right,
            _ => panic!("Invalid input; invalid rotation char"),
        };

        results.push((direction,
                      usize::from_str_radix(dist, 10)
                        .expect("Invalid input; unparseable distance")));
    }

    results
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    East,
    West,
}

impl Facing {
    pub fn turn(&self, r: Rotation) -> Facing {
        use Facing::*;
        match r {
            Rotation::Left => {
                match *self {
                    North => West,
                    West => South,
                    South => East,
                    East => North,
                }
            }
            Rotation::Right => {
                match *self {
                    North => East,
                    East => South,
                    South => West,
                    West => North,
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinates {
    facing: Facing,
    x: isize,
    y: isize,
}

impl Default for Coordinates {
    fn default() -> Coordinates {
        Coordinates {
            facing: Facing::North,
            x: 0,
            y: 0,
        }
    }
}

impl Coordinates {
    pub fn new(facing: Facing, x: isize, y: isize) -> Coordinates {
        Coordinates {
            facing: facing,
            x: x,
            y: y,
        }
    }

    pub fn add(&self, direction: Direction) -> Coordinates {
        let (rotation, distance) = direction;
        let distance = distance as isize;

        let facing = self.facing.turn(rotation);

        let (x, y) = {
            use Facing::*;
            match facing {
                North => (self.x, self.y + distance),
                East => (self.x + distance, self.y),
                South => (self.x, self.y - distance),
                West => (self.x - distance, self.y),
            }
        };

        Coordinates {
            facing: facing,
            x: x,
            y: y,
        }
    }

    pub fn follow(&self, directions: Directions) -> Coordinates {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_default() {
        // test both rotation directions
        assert!(Coordinates::default().add((Rotation::Right, 1)) ==
                Coordinates::new(Facing::East, 1, 0));

        assert!(Coordinates::default().add((Rotation::Left, 1)) ==
                Coordinates::new(Facing::West, -1, 0));

        // test a different distance
        assert!(Coordinates::default().add((Rotation::Right, 5)) ==
                Coordinates::new(Facing::East, 5, 0));

        // test a different initial value
        assert!(Coordinates::new(Facing::East, 0, -5).add((Rotation::Left, 5)) ==
                Coordinates::default());
    }
}
