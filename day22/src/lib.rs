use aoclib::geometry::{Direction, Point};
use regex::Regex;
use std::{
    collections::{HashMap, VecDeque},
    convert::{TryFrom, TryInto},
    io::BufRead,
    ops::IndexMut,
    path::Path,
    rc::Rc,
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

#[derive(Debug, PartialEq, Eq, Clone)]
struct MapNode {
    size: u32,
    used: u32,
    // strictly speaking, this should be a hashset or something, but
    // I'd be very surprised if we ever ended up with a vector more than
    // a handful of items long, so it's probably fine to perform a linear search here.
    data_from: Vec<Point>,
}

impl From<Node> for MapNode {
    fn from(
        Node {
            position,
            size,
            used,
        }: Node,
    ) -> Self {
        MapNode {
            size,
            used,
            data_from: vec![position],
        }
    }
}

impl MapNode {
    fn avail(&self) -> u32 {
        self.size - self.used
    }

    fn move_into(&mut self, other: &mut MapNode) {
        other.used += self.used;
        self.used = 0;
        other.data_from.extend(self.data_from.drain(..));
    }
}

type Map = aoclib::geometry::Map<MapNode>;

fn make_map(input: &Path) -> Result<Map, Error> {
    let nodes: HashMap<_, MapNode> = parse(input)?
        .map(|node| (node.position, node.into()))
        .collect();
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
    Ok(Map::procedural(
        max_x as usize,
        max_y as usize,
        |position| {
            nodes
                .get(&position)
                .expect("input covers all points in map")
                .clone()
        },
    ))
}

struct State {
    map: Map,
    // from parent map, move data from point in direction
    parent: Option<(Rc<State>, Point, Direction)>,
}

impl State {
    fn is_target(&self) -> bool {
        self.map[self.map.bottom_left()]
            .data_from
            .contains(&self.map.bottom_right())
    }

    fn children(self) -> Vec<State> {
        let state_parent = Rc::new(self);
        let mut children = Vec::new();

        state_parent.map.for_each_point(|node, point| {
            for neighbor_point in state_parent.map.orthogonal_adjacencies(point) {
                let neighbor = std::ops::Index::index(&state_parent.map, point);
                if neighbor.avail() >= node.used {
                    let mut map = state_parent.map.clone();
                    let mut neighbor = neighbor.to_owned();
                    map.index_mut(point).move_into(&mut neighbor);
                    map[neighbor_point] = neighbor;

                    children.push(State {
                        map,
                        parent: Some((
                            state_parent.clone(),
                            point,
                            (neighbor_point - point).try_into().expect(
                                "orthogonal adjacencies should always convert to a direction",
                            ),
                        )),
                    })
                }
            }
        });

        children
    }

    fn steps_to(&self) -> usize {
        self.parent
            .as_ref()
            .map(|(parent, _, _)| parent.steps_to() + 1)
            .unwrap_or_default()
    }
}

fn breadth_first_search(map: Map) -> Option<State> {
    let mut queue = VecDeque::new();
    queue.push_back(State { map, parent: None });

    while let Some(state) = queue.pop_front() {
        if state.is_target() {
            return Some(state);
        }
        queue.extend(state.children())
    }

    None
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
    let map = make_map(input)?;
    let solution_state = breadth_first_search(map).ok_or(Error::NoSolution)?;
    println!("min steps to solution: {}", solution_state.steps_to());
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
