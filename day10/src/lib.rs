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

use std::collections::{HashMap, HashSet, VecDeque};

#[macro_use]
extern crate nom;

pub mod parser;

#[derive(Debug)]
pub struct Output(usize);
#[derive(Debug)]
pub struct Bot {
    pub id: usize,
    values: (Option<usize>, Option<usize>),
    cache: Option<(usize, usize)>,
}

type BotInsertErr = String;

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
    pub fn add_value(&mut self, value: usize) -> Result<(), BotInsertErr> {
        self.values = match self.values {
            (None, _) => (Some(value), None),
            (Some(v1), None) => (Some(v1), Some(value)),
            _ => {

                return Err(format!("Can't insert value {} into bot {}; it's full",
                                   value,
                                   self.id));
            }
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

/// A Receiver is a Bot or an Output: it can receive items.
///
/// In either case, it contains the ID of the destination item
pub enum Receiver {
    Bot(usize),
    Output(usize),
}

pub enum Instruction {
    Get { bot_id: usize, value: usize },
    Transfer {
        bot_id: usize,
        low_dest: Receiver,
        high_dest: Receiver,
    },
}

impl Instruction {
    pub fn get(bot_id: usize, value: usize) -> Instruction {
        Instruction::Get {
            bot_id: bot_id,
            value: value,
        }
    }

    pub fn transfer(bot_id: usize, low_dest: Receiver, high_dest: Receiver) -> Instruction {
        Instruction::Transfer {
            bot_id: bot_id,
            low_dest: low_dest,
            high_dest: high_dest,
        }
    }
}


pub type Bots = HashMap<usize, Bot>;
pub type Outputs = HashMap<usize, HashSet<usize>>;

/// Process a list of instructions.
///
/// Be careful--there's no guard currently in place against an incomplete list of instructions
/// leading to an infinite loop.
pub fn process(instructions: Vec<Instruction>) -> Result<(Bots, Outputs), BotInsertErr> {
    let mut bots = Bots::new();
    let mut outputs = Outputs::new();

    // convert to double-ended queue
    let mut instructions: VecDeque<Instruction> = instructions.into_iter().collect();

    while !instructions.is_empty() {
        match instructions.pop_front().unwrap() {
            Instruction::Get { value, bot_id } => {
                bots.entry(bot_id).or_insert(Bot::new(bot_id)).add_value(value)?
            }
            Instruction::Transfer { bot_id, low_dest, high_dest } => {
                // we can't modify bots in-place in the next section, so we use these
                // variables to track any necessary adjustments to the bots
                let mut insert_bot_value_low = None;
                let mut insert_bot_value_high = None;

                if let Some(bot) = bots.get(&bot_id) {
                    if bot.is_full() {
                        match low_dest {
                            Receiver::Bot(id) => {
                                insert_bot_value_low = Some((id, bot.low().unwrap()));
                            }
                            Receiver::Output(id) => {
                                outputs.entry(id)
                                    .or_insert(HashSet::new())
                                    .insert(bot.low().unwrap());
                            }
                        }
                        match high_dest {
                            Receiver::Bot(id) => {
                                insert_bot_value_high = Some((id, bot.high().unwrap()));
                            }
                            Receiver::Output(id) => {
                                outputs.entry(id)
                                    .or_insert(HashSet::new())
                                    .insert(bot.high().unwrap());
                            }
                        }
                    } else {
                        instructions.push_back(Instruction::transfer(bot_id, low_dest, high_dest));
                    }
                } else {
                    instructions.push_back(Instruction::transfer(bot_id, low_dest, high_dest));
                }

                // if everything worked, propagate our values to bots
                if let Some((id, value)) = insert_bot_value_low {
                    bots.entry(id).or_insert(Bot::new(id)).add_value(value)?;
                }
                if let Some((id, value)) = insert_bot_value_high {
                    bots.entry(id).or_insert(Bot::new(id)).add_value(value)?;
                }
            }
        }
    }

    Ok((bots, outputs))
}

/// Return the bot ID which handles the specified values
pub fn find_bot_handling(bots: &Bots, v1: usize, v2: usize) -> Option<usize> {
    // ensure v1 <= v2 for simpler comparisons
    let search_cache = if v1 < v2 {
        Some((v1, v2))
    } else {
        Some((v2, v1))
    };

    for bot in bots.values() {
        if bot.cache == search_cache {
            return Some(bot.id);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;

    fn get_example_instructions() -> Vec<Instruction> {
        use Receiver::*;
        vec![
            Instruction::get(2, 5),
            Instruction::transfer(2,Bot(1), Bot(0)),
            Instruction::get(1, 3),
            Instruction::transfer(1, Output(1), Bot(0)),
            Instruction::transfer(0, Output(2), Output(0)),
            Instruction::get(2, 2),
        ]
    }

    #[test]
    fn test_expected() {
        let mut expected_outputs = Outputs::new();
        expected_outputs.entry(0).or_insert(HashSet::new()).insert(5);
        expected_outputs.entry(1).or_insert(HashSet::new()).insert(2);
        expected_outputs.entry(2).or_insert(HashSet::new()).insert(3);

        match process(get_example_instructions()) {
            Err(errmsg) => panic!(errmsg),
            Ok((bots, outputs)) => {
                println!("Bots:");
                for bot in bots.values() {
                    println!("  {:?}", bot);
                }
                println!("Outputs: {:?}", outputs);

                assert!(outputs == expected_outputs);
                println!("Bot handling 5 and 2: {:?}", find_bot_handling(&bots, 5, 2));
                assert!(find_bot_handling(&bots, 5, 2) == Some(2));
            }
        }
    }
}
