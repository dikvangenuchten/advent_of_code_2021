extern crate aoc_16;
use aoc_16::*;

fn main() {
    println!("Hello, world!");
    let input = read_file("src/input");
    let (part_1, part_2) = aoc_16(&input);
    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", part_2);
}
