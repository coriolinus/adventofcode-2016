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

type Triangle<T> = [T; 3];


pub fn is_possible<T>(t: &Triangle<T>) -> bool
    where T: Copy + Ord + ::std::ops::Add<Output = T>
{
    let mut t = t.clone();
    t.sort();
    t[0] + t[1] > t[2]
}

pub fn parse_line<T, F>(input: &str, parser: &F) -> Option<Triangle<T>>
    where F: Fn(&str) -> Option<T>,
          T: Copy
{
    if let Some(items) = input.split_whitespace().map(parser).collect::<Option<Vec<T>>>() {
        if items.len() == 3 {
            return Some([items[0], items[1], items[2]]);
        }
    }
    None
}

pub fn parse_lines<T, F>(input: &str, parser: &F) -> Option<Vec<Triangle<T>>>
    where F: Fn(&str) -> Option<T>,
          T: Copy
{
    input.lines().map(|line| parse_line(line, parser)).collect()
}

pub fn parse_lines_as_usize(input: &str) -> Option<Vec<Triangle<usize>>> {
    parse_lines(input, &|s: &str| s.parse::<usize>().ok())
}

pub fn count_valid<T>(ts: Vec<Triangle<T>>) -> usize
    where T: Copy + Ord + ::std::ops::Add<Output = T>
{
    ts.iter().filter(|t| is_possible(t)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_not_possible() {
        let t1 = [5, 10, 25];
        let t2 = [10, 25, 5];
        let t3 = [25, 5, 10];

        assert!([t1, t2, t3].iter().all(|t| !is_possible(t)));
    }

    #[test]
    fn test_is_possible() {
        let t1 = [3, 4, 5];
        let t2 = [40, 50, 30];
        let t3 = [100, 80, 60];

        assert!([t1, t2, t3].iter().all(|t| is_possible(t)));
    }
}
