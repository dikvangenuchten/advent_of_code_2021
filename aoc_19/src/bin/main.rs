extern crate aoc_19;
use aoc_19::*;

use std::fs::File;
use std::io::{BufReader, Read};
pub fn read_file(file: &str) -> String {
    let input = File::open(file).unwrap();

    let mut contents = String::new();
    BufReader::new(input).read_to_string(&mut contents).unwrap();

    return contents;
}

fn main() {
    let input_str = read_file("../all_inputs/aoc_19_input.txt");
    let (part_1, part_2) = aoc_19_comp(&input_str);
    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", part_2);
}
