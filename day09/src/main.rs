extern crate day09lib;
use day09lib::decompress;

extern crate util;
use util::get_line;

fn main() {
    println!("Enter compressed data:");
    let compressed = get_line();
    if let Some(decompressed) = decompress(compressed.trim()) {
        println!("Decompressed len: {}", decompressed.len());
    } else {
        println!("Failed to decompress input");
    }
}
