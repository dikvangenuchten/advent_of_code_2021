use std::time::Instant;

use aoc_11::aoc_11_comp;
use aoc_12::aoc_12_comp;
use aoc_13::{aoc_13_comp, read_file};
use aoc_14::aoc_14_comp;
use aoc_15::aoc_15_comp;
use aoc_16::aoc_16_comp;
use aoc_17::aoc_17_comp;
use aoc_18::aoc_18_comp;
use aoc_19::aoc_19_comp;

fn main() {
    println!("Hello, Advent of Code!");
    let start = Instant::now();

    let day_11 = Instant::now();
    println!("Day 11");
    let input_str = read_file(String::from("../all_inputs/aoc_11_input.txt"));
    let (part_1, part_2) = aoc_11_comp(&input_str);
    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", part_2);
    println!("Day 11 took {:?}", day_11.elapsed());

    let day_12 = Instant::now();
    println!("Day 12");
    let input_str = read_file(String::from("../all_inputs/aoc_12_input.txt"));
    let (part_1, part_2) = aoc_12_comp(&input_str);
    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", part_2);
    println!("Day 12 took {:?}", day_12.elapsed());

    let day_13 = Instant::now();
    println!("Day 13");
    let input_str = read_file(String::from("../all_inputs/aoc_13_input.txt"));
    aoc_13_comp(&input_str);

    println!("Day 13 took {:?}", day_13.elapsed());
    let day_14 = Instant::now();
    println!("Day 14");
    let input_str = read_file(String::from("../all_inputs/aoc_14_input.txt"));
    let (part_1, part_2) = aoc_14_comp(&input_str);
    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", part_2);
    println!("Day 14 took {:?}", day_14.elapsed());

    let day_15 = Instant::now();
    println!("Day 15");
    let input_str = read_file(String::from("../all_inputs/aoc_15_input.txt"));
    let (part_1, part_2) = aoc_15_comp(&input_str);
    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", part_2);
    println!("Day 15 took {:?}", day_15.elapsed());

    let day_16 = Instant::now();
    println!("Day 16");
    let input_str = read_file(String::from("../all_inputs/aoc_16_input.txt"));
    let (part_1, part_2) = aoc_16_comp(&input_str);
    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", part_2);
    println!("Day 16 took {:?}", day_16.elapsed());

    let day_17 = Instant::now();
    println!("Day 17");
    let input_str = read_file(String::from("../all_inputs/aoc_17_input.txt"));
    let (part_1, part_2) = aoc_17_comp(&input_str);
    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", part_2);
    println!("Day 17 took {:?}", day_17.elapsed());

    let day_18 = Instant::now();
    println!("Day 18");
    let input_str = read_file(String::from("../all_inputs/aoc_18_input.txt"));
    let (part_1, part_2) = aoc_18_comp(&input_str);
    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", part_2);
    println!("Day 18 took {:?}", day_18.elapsed());

    let day_19 = Instant::now();
    println!("Day 19");
    let input_str = read_file(String::from("../all_inputs/aoc_19_input.txt"));
    let (part_1, part_2) = aoc_19_comp(&input_str);
    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", part_2);
    println!("Day 19 took {:?}", day_19.elapsed());

    println!("All days took {:?}", start.elapsed());
}
