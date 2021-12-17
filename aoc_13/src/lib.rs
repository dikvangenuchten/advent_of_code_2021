use std::cmp::max;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Read};
use std::str::FromStr;
pub fn read_file(file: String) -> String {
    let input = File::open(file).unwrap();

    let mut contents = String::new();
    BufReader::new(input).read_to_string(&mut contents).unwrap();

    return contents;
}

pub enum Fold {
    Y(u32),
    X(u32),
}

impl Fold {
    fn fold(self: &Self, coord: (u32, u32)) -> (u32, u32) {
        match self {
            Fold::X(fold_line) => {
                if &coord.0 > fold_line {
                    return (2 * fold_line - coord.0, coord.1);
                }
                return coord;
            }
            Fold::Y(fold_line) => {
                if &coord.1 > fold_line {
                    return (coord.0, 2 * fold_line - coord.1);
                }
                return coord;
            }
        }
    }
}

impl FromStr for Fold {
    type Err = ();

    fn from_str(s: &str) -> Result<Fold, ()> {
        let (axis, at) = s.trim_start_matches("fold along ").split_once("=").unwrap();

        match axis {
            "x" => Ok(Fold::X(at.parse::<u32>().unwrap())),
            "y" => Ok(Fold::Y(at.parse::<u32>().unwrap())),
            _ => Err(()),
        }
    }
}

pub fn aoc_13_comp(input: &str) {
    aoc_13(input, true);
}

pub fn aoc_13(input: &str, visualize: bool) -> HashSet<(u32, u32)> {
    let (mut coordinates, folds) = parse_inputs(input);

    for fold in folds {
        coordinates = fold_coordinates(coordinates, &fold);
    }

    if visualize {
        visualize_dots(&coordinates);
    }
    return coordinates;
}

pub fn parse_inputs(input: &str) -> (HashSet<(u32, u32)>, Vec<Fold>) {
    let (dots_part, fold_part) = input.split_once("\n\n").unwrap();
    let coordinates = parse_dots(dots_part);

    let folds = fold_part
        .lines()
        .map(|fold| Fold::from_str(fold).unwrap())
        .collect::<Vec<Fold>>();
    return (coordinates, folds);
}

pub fn parse_dots(dots_part: &str) -> HashSet<(u32, u32)> {
    return dots_part
        .lines()
        .map(|coord| coord.split_once(",").unwrap())
        .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
        .collect::<HashSet<(u32, u32)>>();
}

pub fn fold_coordinates(coordinates: HashSet<(u32, u32)>, fold: &Fold) -> HashSet<(u32, u32)> {
    return coordinates
        .into_iter()
        .map(|coord| fold.fold(coord))
        .collect::<HashSet<(u32, u32)>>();
}

pub fn visualize_dots(coordinates: &HashSet<(u32, u32)>) {
    let (max_x, max_y) = coordinates.iter().fold((0, 0), |(max_x, max_y), (x, y)| {
        (max(max_x, *x), max(max_y, *y))
    });

    for y in 0..=max_y {
        for x in 0..=max_x {
            if coordinates.contains(&(x, y)) {
                print!("\u{2588}");
            } else {
                print!("\u{2591}");
            }
        }
        print!("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input() {
        let input_str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        let (mut coordinates, mut folds) = parse_inputs(input_str);
        coordinates = fold_coordinates(coordinates, &folds.remove(0));
        assert_eq!(coordinates.len(), 17);
    }
}
