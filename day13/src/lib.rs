use aoclib::{
    geometry::{
        map::{ContextFrom, ContextInto, Map, Traversable},
        Point,
    },
    parse,
};
use std::{
    collections::{HashSet, VecDeque},
    path::Path,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Tile(bool);

impl ContextFrom<Tile> for Traversable {
    type Context = ();

    fn ctx_from(Tile(is_wall): Tile, _context: &Self::Context) -> Self {
        match is_wall {
            true => Traversable::Obstructed,
            false => Traversable::Free,
        }
    }
}

fn make_cubical_design(favorite_number: i32) -> impl Fn(Point) -> Tile {
    move |Point { x, y }: Point| -> Tile {
        let mut magic = x * x + 3 * x + 2 * x * y + y + y * y;
        magic += favorite_number;
        Tile(magic.count_ones() % 2 != 0)
    }
}

fn make_map(edge_size: usize, favorite_number: i32) -> Map<Tile> {
    Map::procedural(edge_size, edge_size, make_cubical_design(favorite_number))
}

const INITIAL: Point = Point::new(1, 1);
const PART1_GOAL: Point = Point::new(31, 39);

pub fn part1(input: &Path) -> Result<(), Error> {
    for favorite_number in parse::<i32>(input)? {
        let map = make_map(64, favorite_number);
        let path_len = map
            .navigate(INITIAL, PART1_GOAL)
            .ok_or(Error::NoPath(INITIAL, PART1_GOAL))?
            .len();
        println!("number of steps from initial to goal: {}", path_len);
    }
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    for favorite_number in parse::<i32>(input)? {
        let map = make_map(64, favorite_number);

        let mut visited = HashSet::new();
        visited.insert(INITIAL);
        let mut queue = VecDeque::new();
        queue.push_front((0, INITIAL));

        while let Some((steps, position)) = queue.pop_front() {
            if steps > 50 {
                continue;
            }

            map.orthogonal_adjacencies(position)
                .filter(|adj| {
                    !visited.contains(adj) && {
                        let traversable: Traversable = map[*adj].ctx_into(&());
                        traversable == Traversable::Free
                    }
                })
                .for_each(|adj| queue.push_back((steps + 1, adj)));

            visited.insert(position);
        }

        println!("reachable positions in 50 steps: {}", visited.len());
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("no path found from {0:?} to {1:?}")]
    NoPath(Point, Point),
}
