use aoclib::parse;
use lalrpop_util::lalrpop_mod;
use std::{collections::HashSet, path::Path, str::FromStr};

lalrpop_mod!(floor_parser);

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, parse_display::Display, parse_display::FromStr, Hash,
)]
#[display(style = "lowercase")]
enum Element {
    Cobalt,
    Curium,
    Hydrogen,
    Lithium,
    Plutonium,
    Prometheum,
    Ruthenium,
}

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, parse_display::Display, parse_display::FromStr, Hash,
)]
#[display(style = "lowercase")]
enum Gadget {
    Generator,
    Microchip,
}

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, parse_display::Display, parse_display::FromStr, Hash,
)]
#[display("{element} {gadget}")]
#[from_str(regex = r"(?P<element>[a-z]+)(-compatible)? (?P<gadget>\w+)")]
struct Device {
    element: Element,
    gadget: Gadget,
}

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, parse_display::Display, parse_display::FromStr, Hash,
)]
#[display(style = "lowercase")]
enum FloorId {
    First,
    Second,
    Third,
    Fourth,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Floor {
    id: FloorId,
    contents: HashSet<Device>,
}

impl FromStr for Floor {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        floor_parser::FloorParser::new()
            .parse(s)
            .map_err(|err| err.map_token(|token| token.to_string()).into())
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let floors: Vec<Floor> = parse(input)?.collect();
    println!("{:#?}", floors);
    Ok(())
}

pub fn part2(_input: &Path) -> Result<(), Error> {
    unimplemented!()
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Parse(#[from] lalrpop_util::ParseError<usize, String, &'static str>),
}
