extern crate aoc_18;
use aoc_18::*;

use std::fs::File;
use std::io::{BufReader, Read};
pub fn read_file(file: &str) -> String {
    let input = File::open(file).unwrap();

    let mut contents = String::new();
    BufReader::new(input).read_to_string(&mut contents).unwrap();

    return contents;
}

fn main() {
    let input_str = read_file("src/input");
    println!("Part 1: {:?}", aoc_18_part_1(&input_str));
    println!("Part 2: {:?}", aoc_18_part_2(&input_str));
}
