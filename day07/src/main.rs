extern crate day07lib;
use day07lib::{supports_tls, supports_ssl};

extern crate util;
use util::get_lines;

fn count_supports_tls(lines: &str) -> usize {
    lines.lines().filter(|line| supports_tls(line.trim())).count()
}

fn count_supports_ssl(lines: &str) -> usize {
    lines.lines().filter(|line| supports_ssl(line.trim())).count()
}

fn main() {
    println!("Enter ipv7 addresses:");
    let lines = get_lines();
    println!("ABBA count: {}", count_supports_tls(&lines));
    println!("SSL count:  {}", count_supports_ssl(&lines));
}
