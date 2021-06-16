use aoclib::geometry::{
    map::{ContextFrom, Traversable},
    tile::DisplayWidth,
    Map as GenericMap,
};

use std::{collections::HashMap, path::Path};

#[derive(Debug, Clone, Copy, PartialEq, Eq, parse_display::Display, parse_display::FromStr)]
enum Tile {
    #[display("#")]
    Wall,
    #[display(".")]
    Empty,
    #[display("{0}")]
    Poi(u8),
}

impl Tile {
    fn as_poi(&self) -> Option<u8> {
        match *self {
            Self::Poi(poi) => Some(poi),
            _ => None,
        }
    }
}

impl DisplayWidth for Tile {
    const DISPLAY_WIDTH: usize = 1;
}

impl ContextFrom<Tile> for Traversable {
    type Context = ();

    fn ctx_from(t: Tile, _: &Self::Context) -> Self {
        match t {
            Tile::Wall => Traversable::Obstructed,
            Tile::Empty | Tile::Poi(_) => Traversable::Free,
        }
    }
}

type Map = GenericMap<Tile>;

pub fn traveling_salesman(input: &Path, return_to_start: bool) -> Result<usize, Error> {
    let file = std::fs::File::open(input)?;
    let reader = std::io::BufReader::new(file);
    let map = Map::try_from(reader)?;
    let pois: HashMap<_, _> = map
        .points()
        .filter_map(|point| map[point].as_poi().map(|poi| (poi, point)))
        .collect();
    let mut distances = HashMap::new();
    let mut distance_between = |mut a: u8, mut b: u8| {
        // enforce: a is always less than b
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        *distances.entry((a, b)).or_insert_with(|| {
            let a = pois[&a];
            let b = pois[&b];
            map.navigate(a, b)
                .map(|directions| directions.len())
                .unwrap_or(!0)
        })
    };
    let max_poi = *pois.keys().max().ok_or(Error::NoPois)?;
    let mut ordering: Vec<_> = (1..=max_poi).collect();
    let mut min_path_len = !0;

    permutohedron::heap_recursive(&mut ordering, |ordering| {
        let mut path_len = distance_between(0, ordering[0]);
        for window in ordering.windows(2) {
            if path_len > min_path_len {
                return;
            }
            path_len += distance_between(window[0], window[1]);
        }
        if return_to_start {
            path_len += distance_between(ordering.last().copied().unwrap_or_default(), 0);
        }
        min_path_len = min_path_len.min(path_len);
    });

    if min_path_len == !0 {
        return Err(Error::NoSolution);
    }

    Ok(min_path_len)
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let min_path_len = traveling_salesman(input, false)?;
    println!("min path len: {}", min_path_len);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let min_path_len = traveling_salesman(input, true)?;
    println!("min path len (return to start): {}", min_path_len);
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("reading map")]
    MapRead(#[from] aoclib::geometry::map::MapConversionErr),
    #[error("no points of interest found in the input map")]
    NoPois,
    #[error("no solution found")]
    NoSolution,
}
