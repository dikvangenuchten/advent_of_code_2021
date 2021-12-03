use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    println!("{:?}", aoc_3_1());
}

fn aoc_3_1() -> u32{
    let file = File::open("src/input").unwrap();

    let mut one_count = [0; 12];
    let mut count = 0;

    for line in io::BufReader::new(file).lines() {
        for (i, letter) in line.as_ref().unwrap().chars().enumerate() {
            match letter {
                '1' => one_count[i] += 1,
                _ => (),
            }
        }
        count += 1;
    }

    let gamma: u32 = isize::from_str_radix(
        &one_count.map(|x| if x > (count / 2) { "1" } else { "0" }).join(""),
        2,
    ).unwrap() as u32;

    let epsilon: u32 = isize::from_str_radix(
        &one_count.map(|x| if x < (count / 2) { "1" } else { "0" }).join(""),
        2,
    ).unwrap() as u32;

    return epsilon * gamma;
}
