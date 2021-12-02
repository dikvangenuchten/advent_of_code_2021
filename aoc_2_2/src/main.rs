#![feature(test)]
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    println!("{:?}", aoc_2_2());
}

fn parse_line(line: &str) -> (&str, i32) {
    let parsed_line: Vec<&str> = line.split(" ").collect::<Vec<&str>>();
    let direction: &str = parsed_line.first().unwrap();
    let steps: i32 = parsed_line.last().unwrap().parse::<i32>().unwrap();
    (direction, steps)
}

fn aoc_2_2() -> i32 {
    let file = File::open("src/input").unwrap();
    let mut aim: i32 = 0;
    let mut depth: i32 = 0;
    let mut forward: i32 = 0;
    for line in io::BufReader::new(file).lines() {
        let safe_line = line.unwrap();
        println!("{}", safe_line);
        let (direction, steps) = parse_line(&safe_line);
        match direction {
            "forward" => {
                forward += steps;
                depth += aim * steps;
            }
            "down" => {
                aim += steps;
            }
            "up" => {
                aim -= steps;
            }
            &_ => {

            }
        }
    }

    return depth * forward 
}

extern crate test;
use test::Bencher;
#[bench]
fn speed_v1(b: &mut Bencher) {
    b.iter(|| aoc_2_2());
}
