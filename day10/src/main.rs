use util::get_lines;
use day10::{parse_lines, process, find_bot_handling};

fn main() {
    println!("Enter instructions:");
    let lines = get_lines();
    if let Some(instructions) = parse_lines(&lines) {
        if let Ok((bots, outputs)) = process(instructions) {
            if let Some(bot) = find_bot_handling(&bots, 61, 17) {
                println!("Bot {} handles 61, 17", bot);
            } else {
                println!("Couldn't find bot handling 61, 17");
            }

            let zero = outputs.get(&0).unwrap().iter().next().unwrap();
            let one = outputs.get(&1).unwrap().iter().next().unwrap();
            let two = outputs.get(&2).unwrap().iter().next().unwrap();

            let product = zero * one * two;
            println!("Product of bins [0]*[1]*[2]={}", product);
        } else {
            println!("Failed to process instructions");
        }
    } else {
        println!("Could not parse input instructions");
    }
}
