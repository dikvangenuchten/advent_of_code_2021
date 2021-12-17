extern crate aoc_11;

use aoc_11::aoc_11_comp;
use std::fs::File;
use std::io::{BufReader, Read};
fn main() {
    let input = read_file("src/input");
    let (part_1, part_2) = aoc_11_comp(&input);
    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", part_2);
}

pub fn read_file(file: &str) -> String {
    let input = File::open(file).unwrap();

    let mut contents = String::new();
    BufReader::new(input).read_to_string(&mut contents).unwrap();

    return contents;
}
