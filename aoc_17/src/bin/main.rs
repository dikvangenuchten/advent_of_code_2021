use aoc_17::aoc_17;

fn main() {
    let input_str = "target area: x=240..292, y=-90..-57";
    let (best_y, num) = aoc_17(input_str, None);
    println!("Part 1: {:?}", best_y);
    println!("Part 2: {:?}", num);
}
