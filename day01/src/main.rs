
use day01::{parse, Coordinates};

use util::get_line;

fn main() {
    let directions = parse(&get_line().trim());
    let dist = Coordinates::default().follow(&directions).manhattan();
    println!("Dist: {:?}", dist);
    let dupe_dist = Coordinates::default()
        .follow_until_duplicate(&directions)
        .map(|c| c.manhattan());
    println!("Dupe dist: {:?}", dupe_dist);
}
