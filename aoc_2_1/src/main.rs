#![feature(test)]
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Add;

fn main() {
    let end_location = aoc_2_1();
    println!(
        "{:?}",
        end_location.forward as i32 * -end_location.up as i32
    );
}

#[derive(Debug)]
struct Location {
    up: i16,
    forward: i16,
}

impl Add for Location {
    type Output = Location;

    fn add(self, other: Location) -> Location {
        Self {
            up: self.up + other.up,
            forward: self.forward + other.forward,
        }
    }
}

fn location_from_str(line: &str) -> Location {
    let parsed_line: Vec<&str> = line.split(" ").collect::<Vec<&str>>();
    let direction: &str = parsed_line.first().unwrap();
    let steps: i16 = parsed_line.last().unwrap().parse::<i16>().unwrap();
    match direction {
        "forward" => Location {
            up: 0,
            forward: steps,
        },
        "up" => Location {
            up: steps,
            forward: 0,
        },
        "down" => Location {
            up: -steps,
            forward: 0,
        },
        &_ => Location { up: 0, forward: 0 },
    }
}

fn aoc_2_1() -> Location {
    let file = File::open("src/input").unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|line| location_from_str(&line.unwrap()))
        .fold(Location { forward: 0, up: 0 }, |sum, other| sum + other)
}

extern crate test;
use test::Bencher;
#[bench]
fn speed_v1(b: &mut Bencher) {
    b.iter(|| aoc_2_1());
}
