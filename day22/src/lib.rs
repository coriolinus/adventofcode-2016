use aoclib::geometry::{
    map::{ContextInto, Map as GenericMap, Traversable},
    tile::DisplayWidth,
    Direction, Point,
};
use regex::Regex;
use std::{
    collections::HashMap,
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, parse_display::Display)]
enum MapNode {
    #[display(".")]
    Blank,
    #[display("#")]
    Wall,
}

impl ContextInto<Traversable> for MapNode {
    type Context = ();

    fn ctx_into(self, _: &Self::Context) -> Traversable {
        match self {
            Self::Blank => Traversable::Free,
            Self::Wall => Traversable::Obstructed,
        }
    }
}

impl DisplayWidth for MapNode {
    const DISPLAY_WIDTH: usize = 1;
}

impl Default for MapNode {
    fn default() -> Self {
        MapNode::Blank
    }
}

type Map = GenericMap<MapNode>;

// return a complete map, plus a list of empties
fn make_map(input: &Path) -> Result<(Map, Vec<Point>), Error> {
    let nodes: HashMap<_, Node> = parse(input)?.map(|node| (node.position, node)).collect();
    let max_x = nodes
        .keys()
        .map(|position| position.x)
        .max()
        .ok_or(Error::NoInput)?;
    let max_y = nodes
        .keys()
        .map(|position| position.y)
        .max()
        .ok_or(Error::NoInput)?;
    let raw_map = GenericMap::procedural(max_x as usize + 1, max_y as usize + 1, |position| {
        nodes
            .get(&position)
            .expect("input covers all points in map")
            .clone()
    });
    let empties = nodes
        .iter()
        .filter_map(|(position, node)| (node.used == 0).then(move || *position))
        .collect();
    Ok((
        Map::procedural(raw_map.width(), raw_map.height(), |position| {
            if raw_map
                .orthogonal_adjacencies(position)
                .all(|neighbor_pos| raw_map[neighbor_pos].size >= raw_map[position].used)
            {
                MapNode::Blank
            } else {
                MapNode::Wall
            }
        }),
        empties,
    ))
}

pub fn print_map(input: &Path) -> Result<(), Error> {
    let (map, empties) = make_map(input)?;
    println!("map:\n{}", map);
    println!("empties: {:?}", empties);
    Ok(())
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

pub fn part2(input: &Path) -> Result<(), Error> {
    let (map, empties) = make_map(input)?;
    let (min_steps, starting_position) = empties
        .into_iter()
        .filter_map(|starting_position| {
            // first move the blank tile to the left of the goal tile
            let goal_tile = map.bottom_right() + Direction::Left;
            debug_assert_eq!(goal_tile.y, 0);
            let path_to_goal = map.navigate(starting_position, goal_tile)?;

            // dumb optimization: we can print the map and know that there are no obstacles
            // between here and the goal, so just use straight math instead of actually
            // calculating a path
            Some((
                // how this formula works:
                //
                // - move the empty tile to the immediate left of the goal
                //   tile in the most direct route possible
                // - to move the node tile 1 space left and then reset the
                //   state that the empty is directly to its left, we need
                //   5 moves, multiplied until the empty tile is at the left edge
                // - 1 more to move the node tile into the final empty space
                path_to_goal.len() as i32 + (5 * goal_tile.x) + 1,
                starting_position,
            ))
        })
        .min()
        .ok_or(Error::NoSolution)?;
    println!(
        "min steps to solution (starting from {:?}): {}",
        starting_position, min_steps
    );
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("failed to parse input as RawNode")]
    ParseErr,
    #[error("RawNode is not valid: {0:#?}")]
    Invalid(RawNode),
    #[error("no input")]
    NoInput,
    #[error("could not find path to get goal data to origin node")]
    NoSolution,
}
