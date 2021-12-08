#![feature(test)]
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::string::ParseError;
use std::vec;

fn main() {
    println!("Hello, Advent of Code!");
    let (p1, p2) = aoc_8(String::from("src/input"));
    println!("Part 1: {p1}",);
    println!("Part 2: {p2}")
}

struct SegmentDecoder {
    hint_segments: Vec<HashSet<char>>,
    out_segments: Vec<HashSet<char>>,
    decoder: HashMap<u8, HashSet<char>>,
}

impl FromStr for SegmentDecoder {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hints, out) = s.split_once(" | ").unwrap();
        return Ok(Self {
            hint_segments: hints.split(" ").map(|s| s.chars().collect()).collect(),
            out_segments: out.split(" ").map(|s| s.chars().collect()).collect(),
            decoder: HashMap::new(),
        });
    }
}

impl SegmentDecoder {
    fn decode_output(self: &mut Self) -> Vec<u8> {
        let mut unsolved_segments = vec![];

        // Handle easy cases: 1, 4, 7, 8
        for segment in self.hint_segments.iter() {
            match segment.len() {
                2 => drop(self.decoder.insert(1, segment.clone())),
                3 => drop(self.decoder.insert(7, segment.clone())),
                4 => drop(self.decoder.insert(4, segment.clone())),
                7 => drop(self.decoder.insert(8, segment.clone())),
                _ => drop(unsolved_segments.push(segment)),
            };
        }
        assert!(unsolved_segments.len() == 6);

        // Hanlde 9
        let index = unsolved_segments
            .iter()
            .position(|unsolved_segment| self.decoder.get(&4).unwrap().is_subset(unsolved_segment))
            .unwrap();
        self.decoder
            .insert(9, unsolved_segments.swap_remove(index).clone());
        assert!(unsolved_segments.len() == 5);

        // Handle 0
        let detector_0: HashSet<char> = (self.decoder.get(&8).unwrap()
            - self.decoder.get(&4).unwrap())
        .union(self.decoder.get(&1).unwrap())
        .copied()
        .collect();
        let index = unsolved_segments
            .iter()
            .position(|unsolved_segment| detector_0.is_subset(unsolved_segment))
            .unwrap();
        self.decoder
            .insert(0, unsolved_segments.swap_remove(index).clone());
        assert!(unsolved_segments.len() == 4);

        // Handle 3
        let detector_3 = self.decoder.get(&1).unwrap();
        let index = unsolved_segments
            .iter()
            .position(|unsolved_segment| detector_3.is_subset(unsolved_segment))
            .unwrap();
        self.decoder
            .insert(3, unsolved_segments.swap_remove(index).clone());
        assert!(unsolved_segments.len() == 3);

        // Handle 6 (2,3,5)
        let detector_6 = self.decoder.get(&8).unwrap() - self.decoder.get(&3).unwrap();
        let index = unsolved_segments
            .iter()
            .position(|unsolved_segment| detector_6.is_subset(unsolved_segment))
            .unwrap();
        self.decoder
            .insert(6, unsolved_segments.swap_remove(index).clone());
        assert!(unsolved_segments.len() == 2);

        // Handle 2
        let detector_2 = self.decoder.get(&6).unwrap();
        let index = unsolved_segments
            .iter()
            .position(|unsolved_segment| !unsolved_segment.is_subset(detector_2))
            .unwrap();
        self.decoder
            .insert(2, unsolved_segments.swap_remove(index).clone());
        assert!(unsolved_segments.len() == 1);

        // Handle 5
        self.decoder
            .insert(5, unsolved_segments.swap_remove(0).clone());
        assert!(unsolved_segments.len() == 0);

        return self
            .out_segments
            .iter()
            .map(|segment| {
                self.decoder
                    .iter()
                    .find_map(|(key, val)| {
                        if segment.clone() == val.clone() {
                            Some(key)
                        } else {
                            None
                        }
                    })
                    .unwrap()
            })
            .copied()
            .collect();
    }
}

fn aoc_8(file: String) -> (u32, u32) {
    let input = File::open(file).unwrap();

    let displays = BufReader::new(input)
        .lines()
        .map(|l| aoc_8_1_line(l.unwrap()))
        .collect::<Vec<[u8; 4]>>();

    let part_1 = displays
        .iter()
        .flat_map(|s| s)
        .map(|x| match x {
            1 => 1,
            4 => 1,
            7 => 1,
            8 => 1,
            _ => 0,
        })
        .sum::<u32>();

    let part_2 = displays
        .iter()
        .map(|arr| arr.iter().copied().fold(0, |a, b| a * 10 + b as u32))
        .sum::<u32>();

    return (part_1, part_2);
}

fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

fn aoc_8_1_line(input_line: String) -> [u8; 4] {
    let mut decoder = SegmentDecoder::from_str(input_line.as_str()).unwrap();
    return vec_to_array::<u8, 4>(decoder.decode_output());
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_aoc_8_1_line() {
        let output = aoc_8_1_line(String::from(
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
    ));
        let expected_output: [u8; 4] = [8, 11, 11, 4];

        assert_eq!(output[0], expected_output[0]);
        assert_eq!(output[3], expected_output[3]);
    }

    #[rstest]
    #[case("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf", [5,3,5,3])]
    #[case("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe", [8,3,9,4])]
    #[case("edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc", [9,7,8,1])]
    #[case("fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg", [1,1,9,7])]
    #[case("fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb", [9,3,6,1])]
    #[case("aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea", [4,8,7,3])]
    #[case("fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb", [8,4,1,8])]
    #[case("dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe", [4,5,4,8])]
    #[case("bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef", [1,6,2,5])]
    #[case("egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb", [8,7,1,7])]
    #[case("gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce", [4,3,1,5])]
    fn test_parse_line(#[case] input: String, #[case] expected_output: [u8; 4]) {
        let output = aoc_8_1_line(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_aoc_8_1() {
        let example_input = String::from("src/example_input");

        let output = aoc_8(example_input).0;
        assert_eq!(output, 26)
    }

    #[bench]
    fn test_speed(b: &mut Bencher) {
        b.iter(main);
    }

    #[test]
    fn test_day_8() {
        assert_eq!((355, 983030), aoc_8(String::from("src/input")));
    }
}
