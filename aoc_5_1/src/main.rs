use std::cmp::{max, min, Ordering};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
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
            let start_y = min(self.start.y, self.end.y);
            let end_y = max(self.start.y, self.end.y);
            points = (start_y..=end_y)
                .map(|y| construct_point(self.start.x, y))
                .collect::<Vec<Point>>();
        } else {
            // Vertical
            let start_x = min(self.start.x, self.end.x);
            let end_x = max(self.start.x, self.end.x);
            points = (start_x..=end_x)
                .map(|x| construct_point(x, self.start.y))
                .collect::<Vec<Point>>();
        }
        return points;
    }
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

    if start.x == end.x || start.y == end.y {
        return Some(Line {
            start: start,
            end: end,
        });
    }
    return None;
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
        start: construct_point(0, 0),
        end: construct_point(4, 0),
    };

    let expected_points = vec![
        construct_point(0, 0),
        construct_point(1, 0),
        construct_point(2, 0),
        construct_point(3, 0),
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

