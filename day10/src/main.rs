extern crate util;
use util::get_lines;

extern crate day10lib;
use day10lib::{parse_lines, process, find_bot_handling};

fn main() {
    println!("Enter instructions:");
    let lines = get_lines();
    if let Some(instructions) = parse_lines(&lines) {
        if let Ok((bots, _)) = process(instructions) {
            if let Some(bot) = find_bot_handling(&bots, 61, 17) {
                println!("Bot {} handles 61, 17", bot);
            } else {
                println!("Couldn't find bot handling 61, 17");
            }
        } else {
            println!("Failed to process instructions");
        }
    } else {
        println!("Could not parse input instructions");
    }
}
