//! Advent of Code - Day 02 Instructions
//!
//! Bathroom Security
//!
//! You arrive at Easter Bunny Headquarters under cover of darkness. However, you left in such a
//! rush that you forgot to use the bathroom! Fancy office buildings like this one usually have
//! keypad locks on their bathrooms, so you search the front desk for the code.
//!
//! "In order to improve security," the document you find says, "bathroom codes will no longer
//! be written down. Instead, please memorize and follow the procedure below to access the
//! bathrooms."
//!
//! The document goes on to explain that each button to be pressed can be found by starting on
//! the previous button and moving to adjacent buttons on the keypad: U moves up, D moves down,
//! L moves left, and R moves right. Each line of instructions corresponds to one button,
//! starting at the previous button (or, for the first line, the "5" button); press whatever
//! button you're on at the end of each line. If a move doesn't lead to a button, ignore it.
//!
//! You can't hold it much longer, so you decide to figure out the code as you walk to the
//! bathroom. You picture a keypad like this:
//!
//! ```notrust
//! 1 2 3
//! 4 5 6
//! 7 8 9
//! ```
//!
//! Suppose your instructions are:
//!
//! ```notrust
//! ULL
//! RRDDD
//! LURDL
//! UUUUD
//! ```
//!
//! You start at "5" and move up (to "2"), left (to "1"), and left (you can't, and stay on "1"),
//! so the first button is 1.
//!
//! Starting from the previous button ("1"), you move right twice (to "3") and then down three
//! times (stopping at "9" after two moves and ignoring the third), ending up with 9.
//!
//! Continuing from "9", you move left, up, right, down, and left, ending with 8.
//!
//! Finally, you move up four times (stopping at "2"), then down once, ending with 5.
//!
//! So, in this example, the bathroom code is 1985.
//!
//! Your puzzle input is the instructions from the document you found at the front desk.
//! What is the bathroom code?

#[macro_use]
extern crate lazy_static;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

impl Instruction {
    pub fn from_char(ch: char) -> Option<Instruction> {
        use Instruction::*;

        match ch {
            'u' | 'U' => Some(Up),
            'd' | 'D' => Some(Down),
            'l' | 'L' => Some(Left),
            'r' | 'R' => Some(Right),
            _ => None,
        }
    }

    pub fn from_str(s: &str) -> Option<Vec<Instruction>> {
        s.trim().chars().map(|c| Instruction::from_char(c)).collect()
    }
}

pub type Keypad = Vec<Vec<Option<char>>>;

lazy_static! {
    static ref KEYPAD: Keypad = vec![
        vec![None, None, Some('1'), None, None],
        vec![None, Some('2'), Some('3'), Some('4'), None],
        vec![Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
        vec![None, Some('A'), Some('B'), Some('C'), None],
        vec![None, None, Some('D'), None, None],
        ];
}
/// Represents a key on a keypad
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Key {
    x: usize,
    y: usize,
}

impl Key {
    /// Private constructor which allows direct setting
    fn new(x: usize, y: usize) -> Key {
        Key { x: x, y: y }
    }

    /// Construct a Key from anything which appears on KEYPAD
    pub fn from_char(c: char) -> Option<Key> {
        for (y, line) in KEYPAD.iter().enumerate() {
            for (x, key_char) in line.iter().enumerate() {
                if &Some(c) == key_char {
                    return Some(Key::new(x, y));
                }
            }
        }
        None
    }

    pub fn shift(&self, inst: Instruction) -> Key {
        use Instruction::*;

        match inst {
            Up => {
                if self.y > 0 && KEYPAD[self.y - 1][self.x] != None {
                    Key::new(self.x, self.y - 1)
                } else {
                    *self
                }
            }
            Down => {
                if self.y < KEYPAD.len() - 1 && KEYPAD[self.y + 1][self.x] != None {
                    Key::new(self.x, self.y + 1)
                } else {
                    *self
                }
            }
            Left => {
                if self.x > 0 && KEYPAD[self.y][self.x - 1] != None {
                    Key::new(self.x - 1, self.y)
                } else {
                    *self
                }
            }
            Right => {
                if self.x < KEYPAD[self.y].len() - 1 && KEYPAD[self.y][self.x + 1] != None {
                    Key::new(self.x + 1, self.y)
                } else {
                    *self
                }
            }
        }
    }

    pub fn shift_many<Instructions>(&self, insts: Instructions) -> Key
        where Instructions: IntoIterator<Item = Instruction>
    {
        let mut k = *self;
        for inst in insts {
            k = k.shift(inst);
        }
        k
    }
}

impl Default for Key {
    fn default() -> Key {
        Key::from_char('5').expect("Invalid KEYPAD; default not found")
    }
}

impl ::std::fmt::Display for Key {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f,
               "{}",
               KEYPAD[self.y][self.x].expect("Invalid Key; not on keypad"))
    }
}

/// Parse a number of lines into a code.
pub fn decode(lines: &str) -> Option<String> {
    let mut key = Key::default();
    let mut out = String::new();

    for line in lines.lines() {
        if let Some(instructions) = Instruction::from_str(line) {
            key = key.shift_many(instructions);
            out += &key.to_string();
        } else {
            return None;
        }
    }

    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// From the example:
    ///
    /// ```notrust
    /// ULL
    /// RRDDD
    /// LURDL
    /// UUUUD
    /// ```
    ///
    /// produces "5DB3"
    fn test_decode() {
        let lines = "ULL\nRRDDD\nLURDL\nUUUUD\n";
        let result = decode(lines).expect("Decoding failed when it shouldn't");
        println!("Example result (should be '5DB3'): {}", result);
        assert!(&result == "5DB3");
    }
}
