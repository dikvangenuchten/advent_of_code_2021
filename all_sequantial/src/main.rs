use aoc_12::aoc_12_comp;
use aoc_13::{aoc_13_comp, read_file};
use aoc_14::aoc_14_comp;
use aoc_15::aoc_15_comp;
use aoc_16::aoc_16_comp;
use aoc_17::aoc_17_comp;

fn main() {
    println!("Hello, Advent of Code!");

    println!("Day 12");
    let input_str = read_file(String::from("../all_inputs/aoc_12_input.txt"));
    let (part_1, part_2) = aoc_12_comp(&input_str);
    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", part_2);

    println!("Day 13");
    let input_str = read_file(String::from("../all_inputs/aoc_13_input.txt"));
    aoc_13_comp(&input_str);

    println!("Day 14");
    let input_str = read_file(String::from("../all_inputs/aoc_14_input.txt"));
    let (part_1, part_2) = aoc_14_comp(&input_str);
    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", part_2);

    println!("Day 15");
    let input_str = read_file(String::from("../all_inputs/aoc_15_input.txt"));
    let (part_1, part_2) = aoc_15_comp(&input_str);
    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", part_2);

    println!("Day 16");
    let input_str = read_file(String::from("../all_inputs/aoc_16_input.txt"));
    let (part_1, part_2) = aoc_16_comp(&input_str);
    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", part_2);

    println!("Day 17");
    let input_str = read_file(String::from("../all_inputs/aoc_17_input.txt"));
    let (part_1, part_2) = aoc_17_comp(&input_str);
    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", part_2);
}
