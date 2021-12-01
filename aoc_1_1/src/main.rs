#![feature(test)]
use std::fs::File;
use std::io::Read;
use itertools::Itertools;
extern crate test;
use test::Bencher;

fn main() {
    println!("{:?}", aoc_1_1());
    println!("{:?}", aoc_1_1_v2());
}

fn aoc_1_1_v2() -> u16 {
    let mut file = File::open("src/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents
        .lines()
        .map(|s| s.parse().unwrap())
        .tuple_windows::<(u16, u16)>()
        .fold(0, |sum, (prev, next)| {
            if prev < next {
                sum + 1
            } else {
                sum
            }
        });
}

fn aoc_1_1() -> u16 {
    let mut file = File::open("src/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut prev_value: Option<u16> = None;
    let mut count: u16 = 0;
    for line in contents.lines() {
        let value = line.parse::<u16>().unwrap();
        match prev_value {
            Some(prev_value) => count += if prev_value < value { 1 } else { 0 },
            None => (),
        }
        prev_value = Some(value);
    }
    return count;
}

#[bench]
fn speed(b: &mut Bencher) {
    b.iter(|| aoc_1_1());
}

#[bench]
fn speed_v2(b: &mut Bencher) {
    b.iter(|| aoc_1_1_v2());
}
