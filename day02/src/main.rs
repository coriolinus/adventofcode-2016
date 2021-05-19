use day02::decode;

use util::get_lines;

fn main() {
    println!("Enter code lines here:");
    let lines = get_lines();
    println!("Decoded: {:?}", decode(&lines));
}
