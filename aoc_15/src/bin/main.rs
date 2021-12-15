extern crate aoc_15;
use aoc_15::*;

fn main() {
    let input = read_file("src/input");

    let map_part_1 = Map::new(input.clone(), 1);
    let part_1 = calculate_path_cost(map_part_1).unwrap();

    let map_part_2 = Map::new(input.clone(), 5);
    let part_2 = calculate_path_cost(map_part_2).unwrap();

    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", part_2)
}
