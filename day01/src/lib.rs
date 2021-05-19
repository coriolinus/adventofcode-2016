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

use std::{collections::HashSet, path::Path};
use aoclib::{parse, CommaSep, geometry::{Direction, Point, line_segment::LineSegment}};

#[derive(Clone, Copy, Debug, parse_display::Display, parse_display::FromStr)]
enum Turn {
    #[display("L")]
    Left,
    #[display("R")]
    Right,
}

#[derive(Clone, Copy, Debug, parse_display::Display, parse_display::FromStr)]
#[display("{turn}{distance}")]
#[from_str(regex=r" ?(?P<turn>[LR])(?P<distance>\d+)")]
struct Instruction {
    turn: Turn,
    distance: i32,
}

#[cfg(test)]
impl Instruction {
    const fn new(turn: Turn, distance: i32) -> Instruction {
        Instruction { turn, distance }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    facing: Direction,
    location: Point,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            facing: Direction::Up,
            location: Point::default(),
        }
    }
}


impl Position {
    #[cfg(test)]
    fn new(facing: Direction, location: Point) -> Position {
        Position { facing, location }
    }

    fn follow_instruction(&mut self, instruction: Instruction) {
        match instruction.turn {
            Turn::Left  => {
                self.facing.turn_left();
            }
            Turn::Right => {self.facing.turn_right();},
        }
        self.location += LineSegment{direction: self.facing, distance: instruction.distance};
    }

    fn follow(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            self.follow_instruction(*instruction);
        }
    }

    fn step_instruction(&mut self, instruction: Instruction) -> impl '_ + Iterator<Item=Point> {
        match instruction.turn {
            Turn::Left => self.facing.turn_left(),
            Turn::Right => self.facing.turn_right(),
        };

        (0..instruction.distance).map(move |distance| {self.location += LineSegment {direction: self.facing, distance: 1}; self.location})
    }

    fn follow_until_duplicate(&mut self, instructions: &[Instruction]) -> Option<Point> {
        let mut visited = HashSet::new();

        // add current location before moving
        visited.insert(self.location);

        for instruction in instructions {
            for step in self.step_instruction(*instruction) {
                if !visited.insert(step) {
                    return Some(step);
                }
            }
        }
        None
    }
}

pub fn part1(path: &Path) -> Result<(), Error> {
    let instructions = parse::<CommaSep<Instruction>>(path)?.flatten().collect::<Vec<_>>();
    let mut position = Position::default();
    position.follow(&instructions);
    println!("coords of hq: {:#?}", position.location);
    println!("hq manhattan: {}", position.location.manhattan());
    Ok(())
}

pub fn part2(path: &Path) -> Result<(), Error> {
    let instructions = parse::<CommaSep<Instruction>>(path)?.flatten().collect::<Vec<_>>();
    let mut position = Position::default();
    position.follow(&instructions);
    println!("coords of hq: {:#?}", position.location);
    println!("hq manhattan: {}", position.location.manhattan());
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

    const FIRST_CASE: &[Instruction] = &[
        Instruction::new(Turn::Left, 2),
        Instruction::new(Turn::Right, 3),
    ];

    const SECOND_CASE: &[Instruction] = &[
        Instruction::new(Turn::Right, 2),
        Instruction::new(Turn::Right, 2),
        Instruction::new(Turn::Right, 2),
    ];

    const THIRD_CASE: &[Instruction] = &[
        Instruction::new(Turn::Right, 5),
        Instruction::new(Turn::Left, 5),
        Instruction::new(Turn::Right, 5),
        Instruction::new(Turn::Right, 3),
    ];

    const FOURTH_CASE: &[Instruction] = &[
        Instruction::new(Turn::Right, 8),
        Instruction::new(Turn::Right, 4),
        Instruction::new(Turn::Right, 4),
        Instruction::new(Turn::Right, 8),
    ];

    #[test]
    fn test_add_compound() {
        let mut position = Position::default();
        position.follow_instruction(Instruction::new(Turn::Right, 1));
        position.follow_instruction(Instruction::new(Turn::Right, 1));
        assert_eq!(position, Position::new(Direction::Down, Point::new(1, -1)));

        position.follow_instruction(Instruction::new(Turn::Right, 1));
        position.follow_instruction(Instruction::new(Turn::Right, 1));
        assert_eq!(position, Position::default());
    }

    #[test]
    fn test_follow_first() {
        let mut position = Position::default();
        position.follow(FIRST_CASE);
        assert_eq!(position, Position::new(Direction::Up, Point::new(2, 3)));
        assert_eq!(position.location.manhattan(), 5);
    }

    #[test]
    fn test_follow_second() {
        let mut position = Position::default();
        position.follow(SECOND_CASE);
        assert_eq!(position, Position::new(Direction::Left, Point::new(0, -2)));
        assert_eq!(position.location.manhattan(), 2);
    }

    #[test]
    fn test_follow_third() {
        let mut position = Position::default();
        position.follow(THIRD_CASE);
        assert_eq!(position, Position::new(Direction::Down, Point::new(10, 2)));
        assert_eq!(position.location.manhattan(), 12);
    }

    #[test]
    fn test_follow_until_duplicate() {
        let mut position = Position::default();
        let dupe = position.follow_until_duplicate(FOURTH_CASE);
        assert_eq!(position, Position::new(Direction::Up, Point::new(4, 0)));
        assert_eq!(dupe.unwrap().manhattan(), 4);
    }
}
