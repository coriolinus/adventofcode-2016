use aoclib::geometry::Point;
use regex::Regex;
use std::{
    convert::{TryFrom, TryInto},
    io::BufRead,
    path::Path,
    str::FromStr,
};

lazy_static::lazy_static! {
    static ref RAW_NODE_RE: Regex = Regex::new(r"^/dev/grid/node-x(?P<x>\d+)-y(?P<y>\d+)\s+(?P<size>\d+)T\s+(?P<used>\d+)T\s+(?P<avail>\d+)T\s+(?P<use_pct>\d+)%$").unwrap();
}

#[derive(Debug)]
pub struct RawNode {
    x: u32,
    y: u32,
    size: u32,
    used: u32,
    avail: u32,
    use_pct: u32,
}

impl RawNode {
    fn is_valid(&self) -> bool {
        self.size == self.used + self.avail
            && self.use_pct == (self.used as f64 / self.size as f64 * 100.0).floor() as u32
    }
}

impl FromStr for RawNode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = RAW_NODE_RE.captures(s).ok_or(Error::ParseErr)?;
        // these unwraps should be safe given a properly constructed regex and input
        let x = captures.name("x").unwrap().as_str().parse().unwrap();
        let y = captures.name("y").unwrap().as_str().parse().unwrap();
        let size = captures.name("size").unwrap().as_str().parse().unwrap();
        let used = captures.name("used").unwrap().as_str().parse().unwrap();
        let avail = captures.name("avail").unwrap().as_str().parse().unwrap();
        let use_pct = captures.name("use_pct").unwrap().as_str().parse().unwrap();

        Ok(RawNode {
            x,
            y,
            size,
            used,
            avail,
            use_pct,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Node {
    position: Point,
    size: u32,
    used: u32,
}

impl Node {
    fn avail(&self) -> u32 {
        self.size - self.used
    }
}

fn parse(input: &Path) -> Result<impl '_ + Iterator<Item = Node>, Error> {
    let file = std::fs::File::open(input)?;
    let mut reader = std::io::BufReader::new(file);
    // skip the first two lines
    let mut buffer = String::new();
    for _ in 0..2 {
        let _valid = reader.read_line(&mut buffer).is_ok();
        debug_assert!(_valid);
    }
    aoclib::input::parse_reader::<RawNode, _, _>(reader, input.display())
        .map(|iter| iter.map(|raw_node| raw_node.try_into().unwrap()))
        .map_err(Into::into)
}

impl TryFrom<RawNode> for Node {
    type Error = Error;

    fn try_from(value: RawNode) -> Result<Self, Self::Error> {
        value
            .is_valid()
            .then(|| Node {
                position: Point::new(value.x as i32, value.y as i32),
                size: value.size,
                used: value.used,
            })
            .ok_or_else(move || Error::Invalid(value))
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let mut nodes: Vec<_> = parse(input)?.collect();
    nodes.sort_unstable_by_key(|node| node.avail());

    let mut viable_pairs = 0;
    for (idx, node) in nodes.iter().enumerate() {
        for (partner_idx, potential_partner) in nodes.iter().enumerate() {
            if idx != partner_idx && node.used != 0 && node.used <= potential_partner.avail() {
                viable_pairs += 1;
            }
        }
    }

    println!("viable pairs: {}", viable_pairs);
    Ok(())
}

pub fn part2(_input: &Path) -> Result<(), Error> {
    unimplemented!()
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("failed to parse input as RawNode")]
    ParseErr,
    #[error("RawNode is not valid: {0:#?}")]
    Invalid(RawNode),
}
