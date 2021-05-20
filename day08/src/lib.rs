//! Advent of Code - Day 08 Instructions
//!
//! Two-Factor Authentication
//!
//! You come across a door implementing what you can only assume is an implementation of
//! two-factor authentication after a long game of requirements telephone.
//!
//! To get past the door, you first swipe a keycard (no problem; there was one on a nearby desk).
//! Then, it displays a code on a little screen, and you type that code on a keypad. Then,
//! presumably, the door unlocks.
//!
//! Unfortunately, the screen has been smashed. After a few minutes, you've taken everything
//! apart and figured out how it works. Now you just have to work out what the screen would
//! have displayed.
//!
//! The magnetic strip on the card you swiped encodes a series of instructions for the screen;
//! these instructions are your puzzle input. The screen is 50 pixels wide and 6 pixels tall,
//! all of which start off, and is capable of three somewhat peculiar operations:
//!
//! `rect AxB` turns on all of the pixels in a rectangle at the top-left of the screen which is
//! A wide and B tall.
//! `rotate row y=A by B` shifts all of the pixels in row A (0 is the top row) right by B pixels.
//! Pixels that would fall off the right end appear at the left end of the row.
//! `rotate column x=A by B` shifts all of the pixels in column A (0 is the left column) down
//! by B pixels. Pixels that would fall off the bottom appear at the top of the column.
//!
//! For example, here is a simple sequence on a smaller screen:
//!
//! rect 3x2 creates a small rectangle in the top-left corner:
//!
//! ```notrust
//! ###....
//! ###....
//! .......
//! ```
//!
//! rotate column x=1 by 1 rotates the second column down by one pixel:
//!
//! ```notrust
//! #.#....
//! ###....
//! .#.....
//! ```
//!
//! rotate row y=0 by 4 rotates the top row right by four pixels:
//!
//! ```notrust
//! ....#.#
//! ###....
//! .#.....
//! ```
//! rotate column x=1 by 1 again rotates the second column down by one pixel, causing the
//! bottom pixel to wrap back to the top:
//!
//! ```notrust
//! .#..#.#
//! #.#....
//! .#.....
//! ```
//! As you can see, this display technology is extremely powerful, and will soon dominate the
//! tiny-code-displaying-screen market. That's what the advertisement on the back of the display
//! tries to convince you, anyway.
//!
//! There seems to be an intermediate check of the voltage used by the display: after you swipe
//! your card, if the screen did work, how many pixels should be lit?

use aoclib::{
    geometry::{tile::Bool, Map, Point},
    parse,
};
use std::{collections::VecDeque, path::Path};

#[derive(Debug, PartialEq, Eq, Clone, Copy, parse_display::Display, parse_display::FromStr)]
pub enum Instruction {
    #[display("rect {0}x{1}")]
    Rect(usize, usize),
    #[display("rotate row y={0} by {1}")]
    RotateRow(usize, usize),
    #[display("rotate column x={0} by {1}")]
    RotateCol(usize, usize),
}

pub struct Screen(Map<Bool>);

impl Screen {
    pub fn new(width: usize, height: usize) -> Screen {
        Screen(Map::new(width, height))
    }

    pub fn apply(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Rect(width, height) => self.rect(width, height),
            Instruction::RotateCol(col, by) => self.rotate_col(col, by),
            Instruction::RotateRow(row, by) => self.rotate_row(row, by),
        }
    }

    fn rect(&mut self, width: usize, height: usize) {
        // we have to fill in the top left; origin is on the bottom left
        for y in (self.0.height() - height)..self.0.height() {
            for x in 0..width {
                self.0[(x, y)] = true.into();
            }
        }
    }

    fn rotate_col(&mut self, x: usize, by: usize) {
        // first copy the current column, then write it back
        let init = Point::from((x, 0));
        let mut col: VecDeque<_> = self
            .0
            .project(init, 0, 1)
            .map(|point| self.0[point])
            .collect();
        // since we started at the bottom, this rotates the row down
        col.rotate_left(by);

        for (y, value) in col.into_iter().enumerate() {
            self.0[(x, y)] = value;
        }
    }

    fn rotate_row(&mut self, y: usize, by: usize) {
        // first copy the current row, then write it back
        let y = self.0.height() - y - 1;
        let mut row: VecDeque<_> = self
            .0
            .project(Point::new(0, y as i32), 1, 0)
            .map(|point| self.0[point])
            .collect();
        row.rotate_right(by);

        for (x, value) in row.into_iter().enumerate() {
            self.0[(x, y)] = value;
        }
    }

    fn num_pixels_lit(&self) -> usize {
        self.0.iter().filter(|pixel| (**pixel).into()).count()
    }
}

impl Default for Screen {
    fn default() -> Screen {
        Screen::new(50, 6)
    }
}

impl std::fmt::Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

pub fn part1(path: &Path) -> Result<(), Error> {
    let mut screen = Screen::default();
    for instruction in parse::<Instruction>(path)? {
        screen.apply(instruction);
    }
    println!("num pixels lit: {}", screen.num_pixels_lit());
    Ok(())
}

pub fn part2(path: &Path) -> Result<(), Error> {
    let mut screen = Screen::default();
    for instruction in parse::<Instruction>(path)? {
        screen.apply(instruction);
    }
    println!("screen:\n{}", screen);
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const EXAMPLE: &[&str] = &[
        "rect 3x2",
        "rotate column x=1 by 1",
        "rotate row y=0 by 4",
        "rotate column x=1 by 1",
    ];

    #[test]
    /// rect 3x2 creates a small rectangle in the top-left corner:
    ///
    /// ```notrust
    /// ###....
    /// ###....
    /// .......
    /// ```
    ///
    /// rotate column x=1 by 1 rotates the second column down by one pixel:
    ///
    /// ```notrust
    /// #.#....
    /// ###....
    /// .#.....
    /// ```
    ///
    /// rotate row y=0 by 4 rotates the top row right by four pixels:
    ///
    /// ```notrust
    /// ....#.#
    /// ###....
    /// .#.....
    /// ```
    /// rotate column x=1 by 1 again rotates the second column down by one pixel, causing the
    /// bottom pixel to wrap back to the top:
    ///
    /// ```notrust
    /// .#..#.#
    /// #.#....
    /// .#.....
    /// ```
    fn test_example() {
        let expected = &[
            "###....\n###....\n.......\n",
            "#.#....\n###....\n.#.....\n",
            "....#.#\n###....\n.#.....\n",
            ".#..#.#\n#.#....\n.#.....\n",
        ];
        let mut ts = Screen::new(7, 3);
        for (instruction, expect) in EXAMPLE
            .iter()
            .map(|instruction| instruction.parse::<Instruction>().unwrap())
            .zip(expected)
        {
            ts.apply(instruction);
            assert_eq!(&ts.to_string(), expect);
        }
    }

    #[test]
    fn test_parse_instructions() {
        let expected = vec![
            Instruction::Rect(3, 2),
            Instruction::RotateCol(1, 1),
            Instruction::RotateRow(0, 4),
            Instruction::RotateCol(1, 1),
        ];

        for (line, expect) in EXAMPLE.iter().zip(expected) {
            assert_eq!(line.parse::<Instruction>().unwrap(), expect);
        }
    }
}
