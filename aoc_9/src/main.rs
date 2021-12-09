#![feature(binary_heap_into_iter_sorted)]
#![feature(test)]

use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    println!("Hello, world!");
    let input_matrix = read_file(String::from("src/input"));
    let (part_1, part_2) = oac_9(&input_matrix);
    println!("Part 1: {:?}\nPart 2: {:?}", part_1, part_2);
}

fn read_file(file: String) -> Vec<Vec<u8>> {
    let input = File::open(file).unwrap();

    let mut contents = String::new();
    BufReader::new(input).read_to_string(&mut contents).unwrap();

    return parse_input(contents);
}

fn oac_9(input_matrix: &Vec<Vec<u8>>) -> (u32, u32) {
    let lowest_points = find_lowest_poinst(&input_matrix);

    let basins = find_basins(&lowest_points, &input_matrix);

    let basin_sizes: BinaryHeap<_> = basins.iter().map(|b| b.len() as u32).collect();

    return (
        calculate_risk(lowest_points, &input_matrix),
        basin_sizes.into_iter_sorted().take(3).product(),
    );
}

fn find_basins(
    lowest_points: &Vec<(usize, usize)>,
    input_matrix: &Vec<Vec<u8>>,
) -> Vec<HashSet<(usize, usize)>> {
    let mut basins = vec![];
    for seed in lowest_points {
        basins.push(find_basin(seed, input_matrix))
    }
    return basins;
}

fn find_basin(seed: &(usize, usize), input_matrix: &Vec<Vec<u8>>) -> HashSet<(usize, usize)> {
    let mut backlog = vec![seed.clone()];
    let mut basin: HashSet<(usize, usize)> = HashSet::new();
    basin.insert(seed.clone());

    while !backlog.is_empty() {
        let edge = unsafe { backlog.pop().unwrap_unchecked() };

        for neighbour in get_safe_neighbours(edge, input_matrix.len(), input_matrix[edge.0].len()) {
            if !basin.contains(&neighbour) && input_matrix[neighbour.0][neighbour.1] != 9 {
                backlog.push(neighbour);
                basin.insert(neighbour.clone());
            }
        }
    }

    return basin;
}

fn get_safe_neighbours(seed: (usize, usize), len_x: usize, len_y: usize) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];
    let (i, j) = seed.clone();

    if i + 1 < len_x {
        neighbours.push((i + 1, j));
    }
    if 0 <= i as i8 - 1 {
        neighbours.push((i - 1, j));
    }
    if j + 1 < len_y {
        neighbours.push((i, j + 1));
    }
    if 0 <= j as i8 - 1 {
        neighbours.push((i, j - 1));
    }

    return neighbours;
}

fn parse_input(input_str: String) -> Vec<Vec<u8>> {
    return input_str
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect();
}

fn find_lowest_poinst(input_matrix: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut lowest_points = vec![];
    for i in 0..input_matrix.len() {
        for j in 0..input_matrix[0].len() {
            let center = input_matrix[i][j];
            if i + 1 < input_matrix.len() && input_matrix[i + 1][j] <= center {
                continue;
            }
            if 0 <= i as i8 - 1 && input_matrix[i - 1][j] <= center {
                continue;
            }
            if j + 1 < input_matrix[0].len() && input_matrix[i][j + 1] <= center {
                continue;
            }
            if 0 <= j as i8 - 1 && input_matrix[i][j - 1] <= center {
                continue;
            }
            lowest_points.push((i, j));
        }
    }
    return lowest_points;
}

fn calculate_risk(points: Vec<(usize, usize)>, matrix: &Vec<Vec<u8>>) -> u32 {
    return points
        .iter()
        .fold(0, |a, (i, j)| a + 1 + matrix[*i][*j] as u32);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_example_input() {
        let input = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];

        let (part_1, part_2) = oac_9(&input);

        assert_eq!(part_1, 15);
        assert_eq!(part_2, 1134);
    }

    #[test]
    fn test_actual_input() {
        let input_matrix = read_file(String::from("src/input"));
        let (part_1, part_2) = oac_9(&input_matrix);

        assert_eq!(part_1, 545);
        assert_eq!(part_2, 950600);
    }

    #[bench]
    fn bench_day_8(b: &mut Bencher) {
        let input_matrix = read_file(String::from("src/input"));

        b.iter(|| oac_9(&input_matrix));
    }

    #[rstest]
    #[case(vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]], vec![])]
    #[case(vec![vec![1, 2, 2], vec![2, 2, 2], vec![2, 2, 2]], vec![(0, 0)])]
    #[case(vec![vec![2, 1, 2], vec![2, 2, 2], vec![2, 2, 2]], vec![(0, 1)])]
    #[case(vec![vec![2, 2, 1], vec![2, 2, 2], vec![2, 2, 2]], vec![(0, 2)])]
    #[case(vec![vec![2, 2, 2], vec![1, 2, 2], vec![2, 2, 2]], vec![(1, 0)])]
    #[case(vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]], vec![(1, 1)])]
    #[case(vec![vec![2, 2, 2], vec![2, 2, 1], vec![2, 2, 2]], vec![(1, 2)])]
    #[case(vec![vec![2, 2, 2], vec![2, 2, 2], vec![1, 2, 2]], vec![(2, 0)])]
    #[case(vec![vec![2, 2, 2], vec![2, 2, 2], vec![2, 1, 2]], vec![(2, 1)])]
    #[case(vec![vec![2, 2, 2], vec![2, 2, 2], vec![2, 2, 1]], vec![(2, 2)])]
    fn test_example_easy(#[case] input: Vec<Vec<u8>>, #[case] ex_out: Vec<(usize, usize)>) {
        let out = find_lowest_poinst(&input);

        assert_eq!(out, ex_out);
    }

    #[test]
    fn test_parse_input() {
        let input = String::from("000\n000\n000");
        let out = parse_input(input);

        assert_eq!(out, vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]);
    }
}
