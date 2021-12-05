#![feature(iter_zip)]
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

fn main() {
    println!("{:?}", aoc_5_1());
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd)]
struct Point {
    x: u32,
    y: u32,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.x.cmp(&other.x);
        if ord == Ordering::Equal {
            return self.y.cmp(&other.y);
        }
        return ord;
    }
}

#[derive(Debug, PartialEq)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn to_points(self: Self) -> Vec<Point> {
        let points: Vec<Point>;
        if self.start.x == self.end.x {
            // Horizontal
            points = range(self.start.y, self.end.y)
                .map(|y| construct_point(self.start.x, y))
                .collect::<Vec<Point>>();
        } else if self.start.y == self.end.y {
            // Vertical
            points = range(self.start.x, self.end.x)
                .map(|x| construct_point(x, self.start.y))
                .collect::<Vec<Point>>();
        } else {
            // Diagonal
            points = zip(
                range(self.start.x, self.end.x),
                range(self.start.y, self.end.y),
            )
            .map(|(x, y)| construct_point(x, y))
            .collect();
        }
        return points;
    }
}

fn range(start: u32, end: u32) -> Box<dyn Iterator<Item = u32>> {
    if start < end {
        return Box::new(start..=end);
    }
    return Box::new((end..=start).rev());
}

fn construct_point(x: u32, y: u32) -> Point {
    return Point { x: x, y: y };
}

fn parse_point(point: &str) -> Option<Point> {
    let (x, y) = point.split_once(",")?;
    return Some(construct_point(
        x.parse::<u32>().unwrap(),
        y.parse::<u32>().unwrap(),
    ));
}

fn parse_line(line: String) -> Option<Line> {
    let (start_str, end_str) = line.split_once(" -> ")?;

    let start = parse_point(start_str)?;
    let end = parse_point(end_str)?;

    return Some(Line {
        start: start,
        end: end,
    });
}

fn aoc_5_1() -> usize {
    let input = File::open("src/input").unwrap();

    BufReader::new(input)
        .lines()
        .filter_map(|l| parse_line(l.unwrap()))
        .flat_map(|l| l.to_points())
        .fold(HashMap::<Point, u32>::new(), |mut m, x| {
            *m.entry(x).or_insert(0) += 1;
            m
        })
        .iter()
        .filter(|(_, count)| count >= &&2)
        .count()
}

#[test]
fn test_parse_line_hor() {
    let input = String::from("62,963 -> 62,181");

    let expceted_out = Line {
        start: construct_point(62, 963),
        end: construct_point(62, 181),
    };

    let out = parse_line(input);

    assert_eq!(out, Some(expceted_out));
}

#[test]
fn test_parse_line_ver() {
    let input = String::from("963,62 -> 181,62");

    let expceted_out = Line {
        start: construct_point(963, 62),
        end: construct_point(181, 62),
    };

    let out = parse_line(input);

    assert_eq!(out, Some(expceted_out));
}

#[test]
fn test_line_to_points_hor() {
    let line = Line {
        start: construct_point(0, 4),
        end: construct_point(4, 0),
    };

    let expected_points = vec![
        construct_point(0, 4),
        construct_point(1, 3),
        construct_point(2, 2),
        construct_point(3, 1),
        construct_point(4, 0),
    ];

    assert_eq!(line.to_points(), expected_points)
}

#[test]
fn test_line_to_points_ver() {
    let line = Line {
        start: construct_point(0, 0),
        end: construct_point(0, 4),
    };

    let expected_points = vec![
        construct_point(0, 0),
        construct_point(0, 1),
        construct_point(0, 2),
        construct_point(0, 3),
        construct_point(0, 4),
    ];

    assert_eq!(line.to_points(), expected_points)
}

#[test]
fn test_line_to_points_diag() {
    let line = Line {
        start: construct_point(4, 0),
        end: construct_point(0, 4),
    };

    let expected_points = vec![
        construct_point(4, 0),
        construct_point(3, 1),
        construct_point(2, 2),
        construct_point(1, 3),
        construct_point(0, 4),
    ];

    assert_eq!(line.to_points(), expected_points)
}

#[test]
fn test_range() {
    assert_eq!(range(0, 4).collect::<Vec<u32>>(), vec![0, 1, 2, 3, 4]);
    assert_eq!(range(4, 0).collect::<Vec<u32>>(), vec![4, 3, 2, 1, 0]);
}
