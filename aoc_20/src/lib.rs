use std::collections::HashMap;
use std::{fmt, hash};
use std::{ops::Index, str::FromStr, string::ParseError};

use std::fs::File;
use std::io::{BufReader, Read};

const TRUE_CHAR: char = '#';
const FALSE_CHAR: char = '.';

// const TRUE_CHAR: char = '\u{2588}';
// const FALSE_CHAR: char = '\u{2591}';

pub fn read_file(file: &str) -> String {
    let input = File::open(file).unwrap();

    let mut contents = String::new();
    BufReader::new(input).read_to_string(&mut contents).unwrap();

    return contents;
}

pub fn aoc_20_comp(input_str: &str) -> (u32, u32) {
    return (aoc_20(input_str, 2), aoc_20(input_str, 50));
}

pub fn aoc_20(input_str: &str, iterations: u16) -> u32 {
    let input = Input::from_str(&input_str).unwrap();
    let enhanced_image = input.enhance(iterations);
    return enhanced_image.count_light_pixels();
}

#[derive(Debug, PartialEq)]
struct Input {
    image: Image,
    iea: IEA,
}

impl Input {
    fn enhance(self: &Self, iterations: u16) -> Image {
        let mut image = self.image.clone();
        for _ in 0..iterations {
            image = image.enhance(&self.iea);
        }
        return image;
    }
}

impl FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (enhancer_str, image_str) = s.split_once("\n\n").unwrap();

        return Ok(Self {
            image: Image::from_str(image_str)?,
            iea: IEA::from_str(enhancer_str)?,
        });
    }
}

#[derive(Debug, PartialEq)]
struct IEA {
    enhancer: Vec<bool>,
}

impl FromStr for IEA {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(Self {
            enhancer: s.chars().map(|c| c == '#').collect(),
        });
    }
}

fn to_u16(bits: Vec<bool>) -> u16 {
    return bits.iter().fold(0, |acc, &b| acc * 2 + (b as u16));
}

impl IEA {
    fn enhance(self: &Self, pixel: Vec<bool>) -> bool {
        let index = to_u16(pixel) as usize;
        return self.enhancer[index];
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Image {
    hash: HashMap<(i16, i16), bool>,
    background: bool,
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ((min_i, max_i), (min_j, max_j)) = self.get_boundaries();
        let mut out = String::new();
        for i in min_i..max_i {
            for j in min_j..max_j {
                // if self.hash.contains(&(i, j)) {
                if **self.hash.get(&(i, j)).get_or_insert(&self.background) {
                    out.push(TRUE_CHAR);
                } else {
                    out.push(FALSE_CHAR);
                }
            }
            out.push('\n');
        }

        write!(f, "{}", out)
    }
}

impl FromStr for Image {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hash: HashMap<(i16, i16), bool> = s
            .lines()
            .enumerate()
            .flat_map(|(i, c)| {
                c.chars()
                    .enumerate()
                    .map(move |(j, c)| ((i as i16, j as i16), c == '#'))
            })
            .fold(HashMap::<(i16, i16), bool>::new(), |mut m, (idx, c)| {
                m.insert(idx, c);
                m
            });

        let img = Image {
            hash,
            background: false,
        };
        return Ok(img);
    }
}

impl Image {
    fn enhance(self: &Self, enhancer: &IEA) -> Self {
        let mut new_hash = HashMap::<(i16, i16), bool>::with_capacity(self.hash.len());

        let ((min_i, max_i), (min_j, max_j)) = self.get_boundaries();

        for i in min_i - 1..max_i + 1 {
            for j in min_j - 1..max_j + 1 {
                let mut iea_input = vec![];
                for i_off in -1..2 {
                    for j_off in -1..2 {
                        iea_input.push(self[(i + i_off, j + j_off)])
                    }
                }
                new_hash.insert((i, j), enhancer.enhance(iea_input));
            }
        }

        return Self {
            hash: new_hash,
            background: enhancer.enhance(vec![self.background].repeat(9)),
        };
    }

    fn get_boundaries(self: &Self) -> ((i16, i16), (i16, i16)) {
        let min_i = self.hash.iter().min_by_key(|((i, _), _)| i).unwrap().0 .0;
        let min_j = self.hash.iter().min_by_key(|((_, j), _)| j).unwrap().0 .1;
        let max_i = self.hash.iter().max_by_key(|((i, _), _)| i).unwrap().0 .0 + 1;
        let max_j = self.hash.iter().max_by_key(|((_, j), _)| j).unwrap().0 .1 + 1;

        return ((min_i, max_i), (min_j, max_j));
    }

    fn in_boundries(self: &Self, index: (i16, i16)) -> bool {
        let ((min_i, max_i), (min_j, max_j)) = self.get_boundaries();
        return min_i < index.0 && index.0 < max_i && min_j < index.1 && index.1 < max_j;
    }

    fn count_light_pixels(self: &Self) -> u32 {
        return self.hash.iter().filter(|(_, &on)| on).count() as u32;
    }
}

impl Index<(i16, i16)> for Image {
    type Output = bool;

    fn index(&self, index: (i16, i16)) -> &Self::Output {
        return self.hash.get(&index).get_or_insert(&self.background);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#",
        IEA { enhancer: vec![false, false, true, false, true, false, false, true, true, true, true, true, false, true, false, true, false, true, false, true, true, true, false, true, true, false, false, false, false, false, true, true, true, false, true, true, false, true, false, false, true, true, true, false, true, true, true, true, false, false, true, true, true, true, true, false, false, true, false, false, false, false, true, false, false, true, false, false, true, true, false, false, true, true, true, false, false, true, true, true, true, true, true, false, true, true, true, false, false, false, true, true, true, true, false, false, true, false, false, true, true, true, true, true, false, false, true, true, false, false, true, false, true, true, true, true, true, false, false, false, true, true, false, true, false, true, false, false, true, false, true, true, false, false, true, false, true, false, false, false, false, false, false, true, false, true, true, true, false, true, true, true, true, true, true, false, true, true, true, false, true, true, true, true, false, false, false, true, false, true, true, false, true, true, false, false, true, false, false, true, false, false, true, true, true, true, true, false, false, false, false, false, true, false, true, false, false, false, false, true, true, true, false, false, true, false, true, true, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, true, false, false, true, false, false, true, true, false, false, true, false, false, false, true, true, false, true, true, true, true, true, true, false, true, true, true, true, false, true, true, true, true, false, true, false, true, false, false, false, true, false, false, false, false, false, false, false, true, false, false, true, false, true, false, true, false, false, false, true, true, true, true, false, true, true, false, true, false, false, false, false, false, false, true, false, false, true, false, false, false, true, true, false, true, false, true, true, false, false, true, false, false, false, true, true, false, true, false, true, true, false, false, true, true, true, false, true, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, true, false, true, false, true, false, true, true, true, true, false, true, true, true, false, true, true, false, false, false, true, false, false, false, false, false, true, true, true, true, false, true, false, false, true, false, false, true, false, true, true, false, true, false, false, false, false, true, true, false, false, true, false, true, true, true, true, false, false, false, false, true, true, false, false, false, true, true, false, false, true, false, false, false, true, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, true, false, false, true, true, true, true, false, false, true, false, false, false, true, false, true, false, true, false, false, false, true, true, false, false, true, false, true, false, false, true, true, true, false, false, true, true, true, true, true, false, false, false, false, false, false, false, false, true, false, false, true, true, true, true, false, false, false, false, false, false, true, false, false, true]}
)]
    fn test_parse_iea(#[case] input_str: &str, #[case] expected_iea: IEA) {
        assert_eq!(IEA::from_str(input_str), Ok(expected_iea))
    }

    #[rstest]
    #[case("#..#.\n#....\n##..#\n..#..\n..###", 10)]
    fn test_parse_image(#[case] input_str: &str, #[case] expected_light_pixels: u32) {
        assert_eq!(
            Image::from_str(input_str).unwrap().count_light_pixels(),
            expected_light_pixels
        )
    }

    #[rstest]
    #[case("src/example_input", 35)]
    #[case("src/input", 5846)]
    fn test_part_1(#[case] input_file: &str, #[case] expected_count: u32) {
        let input_str = read_file(input_file);
        assert_eq!(aoc_20(&input_str, 2), expected_count);
    }

    #[rstest]
    #[case("src/example_input", 3351)]
    #[case("src/input", 21149)]
    fn test_part_2(#[case] input_file: &str, #[case] expected_count: u32) {
        let input_str = read_file(input_file);
        assert_eq!(aoc_20(&input_str, 50), expected_count);
    }

    #[rstest]
    #[case(vec![false], 0)]
    #[case(vec![true], 1)]
    #[case(vec![true, false], 2)]
    #[case(vec![false, false, false, false, true, false, false, true, false], 18)]
    #[case(vec![false, false, false, true, false, false, false, true, false], 34)]
    fn test_to_u16(#[case] bits: Vec<bool>, #[case] expected_decimal: u16) {
        assert_eq!(to_u16(bits), expected_decimal);
    }
}
