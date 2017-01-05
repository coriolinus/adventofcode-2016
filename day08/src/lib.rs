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
}

impl Default for TinyScreen {
    fn default() -> TinyScreen {
        TinyScreen::new(50, 6)
    }
}

pub enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateCol(usize, usize),
}

impl Instruction {
    pub fn parse<'a>(line: &'a str) -> Option<Instruction> {
        let mut tokens = line.trim().split_whitespace();
        match tokens.next() {
            Some("rect") => Instruction::parse_rect(tokens),
            Some("rotate") => Instruction::parse_rotate(tokens),
            _ => None,
        }
    }

    /// `rect AxB` turns on all of the pixels in a rectangle at the top-left of the screen which is
    /// A wide and B tall.
    fn parse_rect<'a, I>(mut tokens: I) -> Option<Instruction>
        where I: Iterator<Item = &'a str>
    {
        if let Some(maybe_dimensions) = tokens.next() {
            if let Some(x) = maybe_dimensions.find('x') {
                let (width_s, rest) = maybe_dimensions.split_at(x);
                let (_x, height_s) = rest.split_at(1);

                let width_p = width_s.parse::<usize>().ok();
                let height_p = height_s.parse::<usize>().ok();

                match (width_p, height_p) {
                    (Some(width), Some(height)) => {
                        return Some(Instruction::Rect(width, height));
                    }
                    _ => {}
                };
            };
        };
        None
    }

    /// `rotate row y=A by B` shifts all of the pixels in row A (0 is the top row) right by
    /// B pixels. Pixels that would fall off the right end appear at the left end of the row.
    /// `rotate column x=A by B` shifts all of the pixels in column A (0 is the left column) down
    /// by B pixels. Pixels that would fall off the bottom appear at the top of the column.
    fn parse_rotate<'a, I>(mut tokens: I) -> Option<Instruction>
        where I: Iterator<Item = &'a str>
    {
        match tokens.next() {
            Some("row") => {
                if let Some((row, shift)) = Instruction::parse_rot_rest(tokens) {
                    Some(Instruction::RotateRow(row, shift))
                } else {
                    None
                }
            }
            Some("column") => {
                if let Some((col, shift)) = Instruction::parse_rot_rest(tokens) {
                    Some(Instruction::RotateCol(col, shift))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn parse_rot_rest<'a, I>(mut tokens: I) -> Option<(usize, usize)>
        where I: Iterator<Item = &'a str>
    {
        match tokens.next() {
            Some(maybe_which) => {
                let mw_bytes = maybe_which.as_bytes();
                if mw_bytes[..2] == b"x="[..2] || mw_bytes[..2] == b"y="[..2] {
                    let digits = {
                            unsafe { std::str::from_utf8_unchecked(&mw_bytes[2..]) }
                        }
                        .parse::<usize>()
                        .ok();
                    if tokens.next() == Some("by") {
                        if let Some(amount_unparsed) = tokens.next() {
                            match (digits, amount_unparsed.parse::<usize>().ok()) {
                                (Some(which), Some(amount)) => return Some((which, amount)),
                                _ => {}
                            }
                        }
                    }
                }
                None
            }
            _ => None,
        }
    }
}
