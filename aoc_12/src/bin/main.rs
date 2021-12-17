extern crate aoc_12;

use aoc_12::aoc_12_comp;

fn main() {
    println!("Hello, world!");

    let input = String::from(
        "lg-GW
pt-start
pt-uq
nx-lg
ve-GW
start-nx
GW-start
GW-nx
pt-SM
sx-GW
lg-end
nx-SM
lg-SM
pt-nx
end-ve
ve-SM
TG-uq
end-SM
SM-uq",
    );

    let (part_1, part_2) = aoc_12_comp(&input);

    println!("n_paths part 1: {:?}", part_1);
    println!("n_paths part 2: {:?}", part_2);
}
