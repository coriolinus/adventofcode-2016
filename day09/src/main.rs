extern crate day09lib;
use day09lib::{decompress, count_decompressed_v2};

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
    if let Some(decompressed_size) = count_decompressed_v2(&mut compressed.trim().chars()) {
        println!("Full decompression length: {}", decompressed_size);
    } else {
        println!("Failed to count-decompress input");
    }
}
