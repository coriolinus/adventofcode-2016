//! Advent of Code - Day 03 Instructions
//!
//! Squares with three sides
//!
//! Now that you can think clearly, you move deeper into the labyrinth of hallways and office
//! furniture that makes up this part of Easter Bunny HQ. This must be a graphic design
//! department; the walls are covered in specifications for triangles.
//!
//! Or are they?
//!
//! The design document gives the side lengths of each triangle it describes, but... 5 10 25?
//! Some of these aren't triangles. You can't help but mark the impossible ones.
//!
//! In a valid triangle, the sum of any two sides must be larger than the remaining side.
//! For example, the "triangle" given above is impossible, because 5 + 10 is not larger than 25.
//!
//! In your puzzle input, how many of the listed triangles are possible?

use aoclib::parse;
use std::path::Path;

#[derive(Debug, parse_display::Display, parse_display::FromStr)]
#[display("{0:>3} {1:>3} {2:>3}")]
#[from_str(regex = r"(?P<0>\d+)\s+(?P<1>\d+)\s+(?P<2>\d+)")]
struct Triangle(u64, u64, u64);

impl Triangle {
    fn as_array(&self) -> [u64; 3] {
        [self.0, self.1, self.2]
    }

    fn is_possible(&self) -> bool {
        let mut array = self.as_array();
        array.sort_unstable();
        array[0] + array[1] > array[2]
    }
}

#[cfg(test)]
impl From<[u64; 3]> for Triangle {
    fn from([a, b, c]: [u64; 3]) -> Self {
        Triangle(a, b, c)
    }
}

fn reorient(triangles: &[Triangle]) -> Vec<Triangle> {
    let mut vertical = Vec::with_capacity(triangles.len());
    for block in triangles.chunks_exact(3) {
        for vertical_idx in 0..3 {
            vertical.push(Triangle(
                block[0].as_array()[vertical_idx],
                block[1].as_array()[vertical_idx],
                block[2].as_array()[vertical_idx],
            ));
        }
    }
    vertical
}

pub fn part1(path: &Path) -> Result<(), Error> {
    let possible_triangles = parse::<Triangle>(path)?.filter(|t| t.is_possible()).count();
    println!("possible triangles: {}", possible_triangles);
    Ok(())
}

pub fn part2(path: &Path) -> Result<(), Error> {
    let triangles: Vec<Triangle> = parse(path)?.collect();
    let triangles = reorient(&triangles);
    let possible = triangles.iter().filter(|t| t.is_possible()).count();
    println!("possible triangles (vertical orient): {}", possible);
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use aoclib::input::parse_str;

    use super::*;

    #[test]
    fn test_is_not_possible() {
        let t1: Triangle = [5, 10, 25].into();
        let t2: Triangle = [10, 25, 5].into();
        let t3: Triangle = [25, 5, 10].into();

        assert!([t1, t2, t3].iter().all(|t| !t.is_possible()));
    }

    #[test]
    fn test_is_possible() {
        let t1: Triangle = [3, 4, 5].into();
        let t2: Triangle = [40, 50, 30].into();
        let t3: Triangle = [100, 80, 60].into();

        assert!([t1, t2, t3].iter().all(|t| t.is_possible()));
    }

    #[test]
    fn test_parse_vertical() {
        let input = "101 301 501\n102 302 502\n103 303 503\n\
                     201 401 601\n202 402 602\n203 403 603\n";

        let expected = vec![
            [101, 102, 103],
            [301, 302, 303],
            [501, 502, 503],
            [201, 202, 203],
            [401, 402, 403],
            [601, 602, 603],
        ];

        let triangles = parse_str(input).unwrap().collect::<Vec<_>>();
        let triangles = reorient(&triangles);

        for (have, expect) in triangles.iter().zip(expected) {
            assert_eq!(have.as_array(), expect);
        }
    }
}
