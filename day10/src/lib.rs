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

use aoclib::parse;
use std::{
    array,
    collections::{hash_map::Entry, HashMap, VecDeque},
    path::Path,
};

// These typedefs aren't type-safe with each other, but they still
// make it easier to read the code.
pub type Id = u32;
pub type Value = u32;
pub type Bots = HashMap<Id, Bot>;
pub type Outputs = HashMap<Id, Value>;

#[derive(Debug)]
pub struct Output(Id);

#[derive(Debug, Default, Clone)]
pub struct Bot {
    pub id: Id,
    low: Option<Value>,
    high: Option<Value>,
}

impl Bot {
    pub fn new(id: Id) -> Bot {
        Bot {
            id,
            ..Bot::default()
        }
    }

    /// True if bot has two values
    pub fn is_full(&self) -> bool {
        self.low.is_some() && self.high.is_some()
    }

    /// Add a result to this bot, or error if it's full
    pub fn add_value(&mut self, mut value: Value) -> Result<(), Error> {
        if let Some(mut low) = self.low.take() {
            if low > value {
                std::mem::swap(&mut low, &mut value);
            }
            self.low = Some(low);
            self.high = Some(value);
        } else {
            self.low = Some(value);
        }

        Ok(())
    }
}

/// A Receiver is a Bot or an Output: it can receive items.
///
/// In either case, it contains the ID of the destination item
#[derive(Debug, PartialEq, Eq, Clone, Copy, parse_display::FromStr, parse_display::Display)]
pub enum Receiver {
    #[display("bot {0}")]
    Bot(Id),
    #[display("output {0}")]
    Output(Id),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, parse_display::FromStr, parse_display::Display)]
pub enum Instruction {
    #[display("value {value} goes to bot {bot_id}")]
    Get { bot_id: Id, value: Value },
    #[display("bot {bot_id} gives low to {low_dest} and high to {high_dest}")]
    Transfer {
        bot_id: Id,
        low_dest: Receiver,
        high_dest: Receiver,
    },
}

impl Instruction {
    pub const fn get(bot_id: Id, value: Value) -> Instruction {
        Instruction::Get { bot_id, value }
    }

    pub const fn transfer(bot_id: Id, low_dest: Receiver, high_dest: Receiver) -> Instruction {
        Instruction::Transfer {
            bot_id,
            low_dest,
            high_dest,
        }
    }
}

/// Process a list of instructions.
///
/// Be careful--there's no guard currently in place against an incomplete list of instructions
/// leading to an infinite loop.
pub fn process(instructions: &[Instruction]) -> Result<(Bots, Outputs), Error> {
    let mut bots = Bots::new();
    let mut outputs = Outputs::new();

    // convert to double-ended queue
    let mut instructions: VecDeque<Instruction> = instructions.iter().copied().collect();

    while let Some(instruction) = instructions.pop_front() {
        match instruction {
            Instruction::Get { value, bot_id } => bots
                .entry(bot_id)
                .or_insert_with(|| Bot::new(bot_id))
                .add_value(value)?,
            Instruction::Transfer {
                bot_id,
                low_dest,
                high_dest,
            } => {
                // clone the bot here to avoid mutable-immutable borrow issues
                // bots are small; this is cheap
                if let Some(Bot {
                    low: Some(low),
                    high: Some(high),
                    ..
                }) = bots.get(&bot_id).cloned()
                {
                    // transfer instruction and bot is full
                    let mut give_to_receiver = |value, receiver| match receiver {
                        Receiver::Bot(id) => {
                            bots.entry(id).or_insert_with(|| Bot::new(id)).add_value(value)
                        }
                        Receiver::Output(id) => match outputs.entry(id) {
                            Entry::Occupied(entry) => {
                                // it's an error to put two different values into the same output
                                if *entry.get() != value {
                                    Err(Error::OutputInsert(id, *entry.get(), value))
                                } else {
                                    Ok(())
                                }
                            }
                            Entry::Vacant(entry) => {
                                entry.insert(value);
                                Ok(())
                            }
                        },
                    };

                    give_to_receiver(low, low_dest)?;
                    give_to_receiver(high, high_dest)?;
                } else {
                    // bot is not found or not full; try again later
                    instructions.push_back(Instruction::transfer(bot_id, low_dest, high_dest));
                }
            }
        }
    }

    Ok((bots, outputs))
}

/// Return the bot ID which handles the specified values
pub fn find_bot_handling(bots: &Bots, mut low: Value, mut high: Value) -> Result<Id, Error> {
    // ensure v1 <= v2 for simpler comparisons
    if low > high {
        std::mem::swap(&mut low, &mut high);
    }

    bots.values()
        .find(|bot| bot.low == Some(low) && bot.high == Some(high))
        .map(|bot| bot.id)
        .ok_or(Error::NoBotFound(low, high))
}

pub fn part1(path: &Path) -> Result<(), Error> {
    let instructions: Vec<Instruction> = parse(path)?.collect();
    let (bots, _) = process(&instructions)?;
    let bot = find_bot_handling(&bots, 61, 17)?;
    println!("Bot handling (61, 17): {}", bot);
    Ok(())
}

pub fn part2(path: &Path) -> Result<(), Error> {
    let instructions: Vec<Instruction> = parse(path)?.collect();
    let (_, outputs) = process(&instructions)?;
    let chips = array::IntoIter::new([0, 1, 2])
        .map(|id| outputs.get(&id).ok_or(Error::NoChipFound(id)))
        .collect::<Result<Vec<_>, _>>()?;
    let chip_product: Value = chips.into_iter().product();
    println!("Product of chips (0, 1, 2): {}", chip_product);
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("bot {1} is full but attempted to insert {0}")]
    BotInsert(Value, Id),
    #[error("could not find bot handling ({0}, {1})")]
    NoBotFound(Value, Value),
    #[error("output {0} contains {1} but attempted to insert {2}")]
    OutputInsert(Id, Value, Value),
    #[error("could not find a chip output {0}")]
    NoChipFound(Id),
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;

    const EXAMPLE_INSTRUCTIONS_STR: &[&str] = &[
        "value 5 goes to bot 2",
        "bot 2 gives low to bot 1 and high to bot 0",
        "value 3 goes to bot 1",
        "bot 1 gives low to output 1 and high to bot 0",
        "bot 0 gives low to output 2 and high to output 0",
        "value 2 goes to bot 2",
    ];

    const EXAMPLE_INSTRUCTIONS: &[Instruction] = &[
        Instruction::get(2, 5),
        Instruction::transfer(2, Receiver::Bot(1), Receiver::Bot(0)),
        Instruction::get(1, 3),
        Instruction::transfer(1, Receiver::Output(1), Receiver::Bot(0)),
        Instruction::transfer(0, Receiver::Output(2), Receiver::Output(0)),
        Instruction::get(2, 2),
    ];

    #[test]
    fn test_expected() {
        let expected_outputs = hashmap!{
            0 => 5,
            1 => 2,
            2 => 3,
        };

        let (bots, outputs) = process(EXAMPLE_INSTRUCTIONS).unwrap();

        println!("Bots:");
        for bot in bots.values() {
            println!("  {:?}", bot);
        }
        println!("Outputs: {:?}", outputs);

        assert!(outputs == expected_outputs);
        assert_eq!(find_bot_handling(&bots, 5, 2).unwrap(), 2);
    }

    #[test]
    fn test_parse() {
        for (raw, parsed) in EXAMPLE_INSTRUCTIONS_STR
            .iter()
            .zip(EXAMPLE_INSTRUCTIONS.iter())
        {
            println!("Parsing '{}'; expecting {:?}", raw, parsed);
            let got = raw.parse::<Instruction>().unwrap();
            assert_eq!(got, *parsed);
        }
    }
}
