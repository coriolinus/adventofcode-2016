//! Advent of Code - Day 10 Instructions
//!
//! Balance Bots
//!
//! You come upon a factory in which many robots are zooming around handing small microchips
//! to each other.
//!
//! Upon closer examination, you notice that each bot only proceeds when it has two microchips,
//! and once it does, it gives each one to a different bot or puts it in a marked "output" bin.
//! Sometimes, bots take microchips from "input" bins, too.
//!
//! Inspecting one of the microchips, it seems like they each contain a single number; the bots
//! must use some logic to decide what to do with each chip. You access the local control
//! computer and download the bots' instructions (your puzzle input).
//!
//! Some of the instructions specify that a specific-valued microchip should be given to a
//! specific bot; the rest of the instructions indicate what a given bot should do with its
//! lower-value or higher-value chip.
//!
//! For example, consider the following instructions:
//!
//! ```notrust
//! value 5 goes to bot 2
//! bot 2 gives low to bot 1 and high to bot 0
//! value 3 goes to bot 1
//! bot 1 gives low to output 1 and high to bot 0
//! bot 0 gives low to output 2 and high to output 0
//! value 2 goes to bot 2
//! ```
//!
//! - Initially, bot 1 starts with a value-3 chip, and bot 2 starts with a value-2 chip and
//! a value-5 chip.
//! - Because bot 2 has two microchips, it gives its lower one (2) to bot 1 and its higher
//! one (5) to bot 0.
//! - Then, bot 1 has two microchips; it puts the value-2 chip in output 1 and gives the
//! value-3 chip to bot 0.
//! - Finally, bot 0 has two microchips; it puts the 3 in output 2 and the 5 in output 0.
//!
//! In the end, output bin 0 contains a value-5 microchip, output bin 1 contains a value-2
//! microchip, and output bin 2 contains a value-3 microchip. In this configuration, bot
//! number 2 is responsible for comparing value-5 microchips with value-2 microchips.
//!
//! Based on your instructions, what is the number of the bot that is responsible for
//! comparing value-61 microchips with value-17 microchips?
#[macro_use]
extern crate nom;

pub mod parser;

pub struct Output(usize);
pub struct Bot {
    pub id: usize,
    values: (Option<usize>, Option<usize>),
    cache: Option<(usize, usize)>,
}

impl Bot {
    pub fn new(id: usize) -> Bot {
        Bot {
            id: id,
            values: (None, None),
            cache: None,
        }
    }

    /// True if bot has two values
    pub fn is_full(&self) -> bool {
        self.values.0.is_some() && self.values.1.is_some()
    }

    /// Add a result to this bot, or error if it's full
    pub fn add_value(&mut self, value: usize) -> Result<(), ()> {
        self.values = match self.values {
            (None, _) => (Some(value), None),
            (Some(v1), None) => (Some(v1), Some(value)),
            _ => return Err(()),
        };

        if self.is_full() {
            let v1 = self.values.0.unwrap();
            let v2 = self.values.1.unwrap();

            if v1 < v2 {
                self.cache = Some((v1, v2));
            } else {
                self.cache = Some((v2, v1));
            }
        }

        Ok(())
    }

    pub fn low(&self) -> Option<usize> {
        self.cache.map(|(l, _)| l)
    }

    pub fn high(&self) -> Option<usize> {
        self.cache.map(|(_, h)| h)
    }
}

/// A Receiver is a Bot or an Output: it can receive items
pub enum ReceiverType {
    Bot,
    Output,
}

pub struct Receiver {
    rtype: ReceiverType,
    id: usize,
}

pub enum Instruction {
    Get { value: usize, bot: Bot },
    Transfer {
        origin: Bot,
        low_dest: Receiver,
        high_dest: Receiver,
    },
}


type BotMap = std::collections::HashMap<usize, Bot>;

pub fn process(instructions: Vec<Instruction>) -> Result<BotMap, ()> {
    unimplemented!()
}
