use aoclib::parse;

use std::{convert::TryInto, iter, path::Path, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, parse_display::FromStr, parse_display::Display)]
enum Tile {
    #[display(".")]
    Safe,
    #[display("^")]
    Trap,
}

fn tiles_from_str(s: &str) -> Result<Vec<Tile>, Error> {
    s.as_bytes()
        .windows(1)
        .map(|s| -> Result<Tile, Error> {
            Tile::from_str(std::str::from_utf8(s)?).map_err(Into::into)
        })
        .collect()
}

fn tile_groups(tiles: &[Tile]) -> impl '_ + Iterator<Item = [Tile; 3]> {
    iter::once([Tile::Safe, tiles[0], tiles[1]])
        .chain(tiles.windows(3).map(|window| {
            // try_into willl always succeed here because windows won't generate a wrong-size slice
            window.try_into().unwrap()
        }))
        .chain(iter::once([
            tiles[tiles.len() - 2],
            tiles[tiles.len() - 1],
            Tile::Safe,
        ]))
}

fn next_row(tiles: &[Tile]) -> Vec<Tile> {
    let mut row = Vec::with_capacity(tiles.len());
    for group in tile_groups(tiles) {
        use Tile::*;
        row.push(match group {
            [Trap, Trap, Safe] | [Safe, Trap, Trap] | [Trap, Safe, Safe] | [Safe, Safe, Trap] => {
                Trap
            }
            _ => Safe,
        });
    }
    debug_assert_eq!(row.len(), tiles.len());
    row
}

fn count_safe_in_n_rows(tiles: &[Tile], n: usize) -> usize {
    let mut safe = 0;
    let mut row = tiles.to_vec();

    for _ in 0..n {
        safe += row.iter().filter(|tile| **tile == Tile::Safe).count();
        row = next_row(&row);
    }

    safe
}

pub fn part1(input: &Path) -> Result<(), Error> {
    for initial_row in parse::<String>(input)?.map(|row| tiles_from_str(&row)) {
        let safe_tiles = count_safe_in_n_rows(&initial_row?, 40);
        println!("safe tiles: {}", safe_tiles);
    }
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    for initial_row in parse::<String>(input)?.map(|row| tiles_from_str(&row)) {
        let safe_tiles = count_safe_in_n_rows(&initial_row?, 400_000);
        println!("safe tiles 400k: {}", safe_tiles);
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Utf8(#[from] std::str::Utf8Error),
    #[error(transparent)]
    ParseDisplay(#[from] parse_display::ParseError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_groups() {
        use Tile::*;

        let tiles = tiles_from_str("..^^.").unwrap();
        let expect = [
            [Safe, Safe, Safe],
            [Safe, Safe, Trap],
            [Safe, Trap, Trap],
            [Trap, Trap, Safe],
            [Trap, Safe, Safe],
        ];

        assert_eq!(tile_groups(&tiles).collect::<Vec<_>>(), &expect);
    }

    #[test]
    fn test_next_row() {
        let tiles = tiles_from_str("..^^.").unwrap();
        let expect = tiles_from_str(".^^^^").unwrap();
        assert_eq!(next_row(&tiles), expect);

        let subsequent = tiles_from_str("^^..^").unwrap();
        assert_eq!(next_row(&expect), subsequent);
    }

    #[test]
    fn test_big_example() {
        let tiles = tiles_from_str(".^^.^.^^^^").unwrap();
        assert_eq!(count_safe_in_n_rows(&tiles, 10), 38);
    }
}
