#![feature(test)]
use std::fs::File;
use std::io::Read;
use itertools::Itertools;
extern crate test;
use test::Bencher;

fn main() {
    aoc_1_2();
    aoc_1_2_v2();
}

fn aoc_1_2_v2() -> u16 {
    let mut file = File::open("src/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents
        .lines()
        .map(|s| s.parse::<u16>().unwrap())
        .tuple_windows::<(u16, u16, u16)>()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows::<(u16, u16)>()
        .fold(0, |sum, (prev, next)| {
            if prev < next {
                sum + 1
            } else {
                sum
            }
        });
}

fn aoc_1_2() -> u16 {
    let mut file = File::open("src/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let input: Vec<u16> = contents
        .lines()
        .map(|s| s.parse::<u16>().unwrap())
        .collect();
    let mut count: u16 = 0;
    for i in 0..input.len() - 3 {
        let prev_sum: u16 = input[i..i + 3].iter().sum();
        let next_sum: u16 = input[i + 1..i + 4].iter().sum();
        if prev_sum < next_sum {
            count += 1;
        }
    }
    return count
}

#[bench]
fn speed_v1(b: &mut Bencher) {
    b.iter(|| aoc_1_2());
}

#[bench]
fn speed_v2(b: &mut Bencher) {
    b.iter(|| aoc_1_2_v2());
}

#[test]
fn equal(){
    assert_eq!(aoc_1_2(), aoc_1_2_v2())
}