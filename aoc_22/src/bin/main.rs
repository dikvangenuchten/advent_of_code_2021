extern crate aoc_22;
use aoc_22::*;

use std::fs::File;
use std::io::{BufReader, Read};
use std::time::Instant;
pub fn read_file(file: &str) -> String {
    let input = File::open(file).unwrap();

    let mut contents = String::new();
    BufReader::new(input).read_to_string(&mut contents).unwrap();

    return contents;
}

fn main() {
    let day_22 = Instant::now();
    println!("Day 22");
    let input_str = read_file("../all_inputs/aoc_22_input.txt");
    let (part_1, part_2) = aoc_22_comp(&input_str);
    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", part_2);
    println!("Day 22 took {:?}\n\n", day_22.elapsed());
}
