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

pub mod instruction;
use instruction::Instruction;

#[derive(Clone, PartialEq, Eq)]
pub struct TinyScreen {
    width: usize,
    height: usize,
    pixels: Vec<Vec<bool>>,
}

impl TinyScreen {
    pub fn new(width: usize, height: usize) -> TinyScreen {
        TinyScreen {
            width: width,
            height: height,
            pixels: vec![vec![false; width]; height],
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn apply(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Rect(width, height) => self.rect(width, height),
            Instruction::RotateCol(col, by) => self.rotate_col(col, by),
            Instruction::RotateRow(row, by) => self.rotate_row(row, by),
        }
    }

    pub fn rect(&mut self, width: usize, height: usize) {
        for row in 0..height {
            for col in 0..width {
                self.pixels[row][col] = true;
            }
        }
    }

    pub fn rotate_col(&mut self, col: usize, by: usize) {
        // first copy the current column, then write it back
        let mut current_column = Vec::with_capacity(self.height);
        for row in 0..self.height {
            current_column.push(self.pixels[row][col]);
        }
        for row in 0..self.height {
            self.pixels[(row + by) % self.height][col] = current_column[row];
        }
    }

    pub fn rotate_row(&mut self, row: usize, by: usize) {
        // first copy the current row, then write it back
        let current_row = self.pixels[row].clone();
        for col in 0..self.width {
            self.pixels[row][(col + by) % self.width] = current_row[col];
        }
    }
}

impl Default for TinyScreen {
    fn default() -> TinyScreen {
        TinyScreen::new(50, 6)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::instruction::Instruction;

    pub fn get_example_instructions() -> Vec<&'static str> {
        vec![
                "rect 3x2",
                "rotate column x=1 by 1",
                "rotate row y=0 by 4",
                "rotate column x=1 by 1",
            ]
    }

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
    fn test_example() {
        let expected = vec![
            TinyScreen {
                width: 7,
                height: 3,
                pixels: vec![
                    vec![true, true, true, false, false, false, false],
                    vec![true, true, true, false, false, false, false],
                    vec![false, false, false, false, false, false, false],
                ],
            },
            TinyScreen {
                width: 7,
                height: 3,
                pixels: vec![
                    vec![true, false, true, false, false, false, false],
                    vec![true, true, true, false, false, false, false],
                    vec![false, true, false, false, false, false, false],
                ],
            },
            TinyScreen {
                width: 7,
                height: 3,
                pixels: vec![
                    vec![false, false, false, false, true, false, true],
                    vec![true, true, true, false, false, false, false],
                    vec![false, true, false, false, false, false, false],
                ],
            },
            TinyScreen {
                width: 7,
                height: 3,
                pixels: vec![
                    vec![false, true, false, false, true, false, true],
                    vec![true, false, true, false, false, false, false],
                    vec![false, true, false, false, false, false, false],
                ],
            },
                            ];
        let mut ts = TinyScreen::new(7, 3);
        for (instruction, expect) in get_example_instructions()
            .iter()
            .map(|i| Instruction::parse(i).unwrap())
            .zip(expected) {
            ts.apply(instruction);
            assert!(ts == expect);
        }
    }
}
