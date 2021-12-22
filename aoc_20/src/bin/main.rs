extern crate aoc_20;
use aoc_20::*;

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
    let day_20 = Instant::now();
    println!("Day 20");
    let input_str = read_file("../all_inputs/aoc_20_input.txt");
    let (part_1, part_2) = aoc_20_comp(&input_str);
    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", part_2);
    println!("Day 20 took {:?}\n\n", day_20.elapsed());
}
