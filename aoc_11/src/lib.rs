#![feature(test)]
#![feature(map_first_last)]
#![feature(destructuring_assignment)]
use std::collections::{BTreeSet, HashSet};

use std::fs::File;
use std::io::{BufReader, Read};
pub fn read_file(file: String) -> String {
    let input = File::open(file).unwrap();

    let mut contents = String::new();
    BufReader::new(input).read_to_string(&mut contents).unwrap();

    return contents;
}

pub fn aoc_11_comp(input: &str) -> (u32, u32) {
    let input: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let flashes = steps(input.clone(), 100);

    let steps_until_all = steps_until_all_flash(input);

    return (flashes, steps_until_all);
}

fn steps(mut population: Vec<Vec<u8>>, n_steps: u32) -> u32 {
    let mut flashes = 0;
    for _ in 0..n_steps {
        let flashes_step;
        (population, flashes_step) = step(population);
        flashes += flashes_step;
    }
    return flashes;
}

fn steps_until_all_flash(mut population: Vec<Vec<u8>>) -> u32 {
    let mut step_count = 0;
    while !population.iter().all(|vec| vec.iter().all(|x| x == &0)) {
        population = step(population).0;
        step_count += 1;
    }
    return step_count;
}

fn step(before: Vec<Vec<u8>>) -> (Vec<Vec<u8>>, u32) {
    let after_increase = increase_by_one(before);
    let (after_flash, flashes) = flash_al_nines(after_increase);
    return (after_flash, flashes);
}

fn increase_by_one(mut before: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    for i in 0..before.len() {
        for j in 0..before[0].len() {
            before[i][j] += 1;
        }
    }
    return before;
}

fn flash_al_nines(mut before: Vec<Vec<u8>>) -> (Vec<Vec<u8>>, u32) {
    let mut flash_locs = HashSet::new();
    let mut to_be_checked = BTreeSet::new();

    for i in 0..before.len() {
        for j in 0..before[0].len() {
            if before[i][j] > 9 {
                to_be_checked.insert((i, j));
            }
        }
    }

    while !to_be_checked.is_empty() {
        let idx = to_be_checked.pop_first().unwrap();
        if before[idx.0][idx.1] > 9 {
            flash_locs.insert(idx);
            for neighbour in retrieve_safe_neighbours(idx.0, idx.1, before.len(), before[0].len()) {
                if !flash_locs.contains(&neighbour) {
                    before[neighbour.0][neighbour.1] += 1;
                    to_be_checked.insert(neighbour);
                }
            }
        }
    }

    let flashes = flash_locs.len() as u32;
    for flashed in flash_locs.drain() {
        before[flashed.0][flashed.1] = 0;
    }

    return (before, flashes);
}

fn retrieve_safe_neighbours(i: usize, j: usize, max_i: usize, max_j: usize) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];
    let max_i = max_i as i32;
    let max_j = max_j as i32;

    for i_off in -1..=1 {
        for j_off in -1..=1 {
            let neighbour_i = i as i32 + i_off;
            let neighbour_j = j as i32 + j_off;
            if 0 <= neighbour_i && neighbour_i < max_i && 0 <= neighbour_j && neighbour_j < max_j {
                neighbours.push((neighbour_i as usize, neighbour_j as usize));
            }
        }
    }
    return neighbours;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    extern crate test;
    use test::Bencher;

    #[rstest]
    #[case(
        vec![vec![1,1,1,1,1], vec![1,9,9,9,1], vec![1,9,1,9,1], vec![1,9,9,9,1], vec![1,1,1,1,1],],
        vec![vec![3,4,5,4,3], vec![4,0,0,0,4], vec![5,0,0,0,5], vec![4,0,0,0,4], vec![3,4,5,4,3],]
    )]
    #[case(
        vec![vec![3,4,5,4,3], vec![4,0,0,0,4], vec![5,0,0,0,5], vec![4,0,0,0,4], vec![3,4,5,4,3],],
        vec![vec![4,5,6,5,4], vec![5,1,1,1,5], vec![6,1,1,1,6], vec![5,1,1,1,5], vec![4,5,6,5,4],]
    )]
    #[case(vec![vec![9,9,9], vec![9,1,9], vec![9,9,9]], vec![vec![0,0,0], vec![0,0,0], vec![0,0,0]])]
    fn test_simple_example(#[case] before: Vec<Vec<u8>>, #[case] after: Vec<Vec<u8>>) {
        assert_eq!(step(before).0, after);
    }

    #[rstest]
    #[case(vec![vec![5,4,8,3,1,4,3,2,2,3],
vec![2,7,4,5,8,5,4,7,1,1],
vec![5,2,6,4,5,5,6,1,7,3],
vec![6,1,4,1,3,3,6,1,4,6],
vec![6,3,5,7,3,8,5,4,7,8],
vec![4,1,6,7,5,2,4,6,4,5],
vec![2,1,7,6,8,4,1,7,2,1],
vec![6,8,8,2,8,8,1,1,3,4],
vec![4,8,4,6,8,4,8,5,5,4],
vec![5,2,8,3,7,5,1,5,2,6],], vec![vec![6,5,9,4,2,5,4,3,3,4],
vec![3,8,5,6,9,6,5,8,2,2],
vec![6,3,7,5,6,6,7,2,8,4],
vec![7,2,5,2,4,4,7,2,5,7],
vec![7,4,6,8,4,9,6,5,8,9],
vec![5,2,7,8,6,3,5,7,5,6],
vec![3,2,8,7,9,5,2,8,3,2],
vec![7,9,9,3,9,9,2,2,4,5],
vec![5,9,5,7,9,5,9,6,6,5],
vec![6,3,9,4,8,6,2,6,3,7],])]
    #[case(vec![vec![6,5,9,4,2,5,4,3,3,4],
vec![3,8,5,6,9,6,5,8,2,2],
vec![6,3,7,5,6,6,7,2,8,4],
vec![7,2,5,2,4,4,7,2,5,7],
vec![7,4,6,8,4,9,6,5,8,9],
vec![5,2,7,8,6,3,5,7,5,6],
vec![3,2,8,7,9,5,2,8,3,2],
vec![7,9,9,3,9,9,2,2,4,5],
vec![5,9,5,7,9,5,9,6,6,5],
vec![6,3,9,4,8,6,2,6,3,7],], vec![vec![8,8,0,7,4,7,6,5,5,5],
vec![5,0,8,9,0,8,7,0,5,4],
vec![8,5,9,7,8,8,9,6,0,8],
vec![8,4,8,5,7,6,9,6,0,0],
vec![8,7,0,0,9,0,8,8,0,0],
vec![6,6,0,0,0,8,8,9,8,9],
vec![6,8,0,0,0,0,5,9,4,3],
vec![0,0,0,0,0,0,7,4,5,6],
vec![9,0,0,0,0,0,0,8,7,6],
vec![8,7,0,0,0,0,6,8,4,8],])]
    #[case(vec![vec![8,8,0,7,4,7,6,5,5,5],
vec![5,0,8,9,0,8,7,0,5,4],
vec![8,5,9,7,8,8,9,6,0,8],
vec![8,4,8,5,7,6,9,6,0,0],
vec![8,7,0,0,9,0,8,8,0,0],
vec![6,6,0,0,0,8,8,9,8,9],
vec![6,8,0,0,0,0,5,9,4,3],
vec![0,0,0,0,0,0,7,4,5,6],
vec![9,0,0,0,0,0,0,8,7,6],
vec![8,7,0,0,0,0,6,8,4,8],], vec![vec![0,0,5,0,9,0,0,8,6,6],
vec![8,5,0,0,8,0,0,5,7,5],
vec![9,9,0,0,0,0,0,0,3,9],
vec![9,7,0,0,0,0,0,0,4,1],
vec![9,9,3,5,0,8,0,0,6,3],
vec![7,7,1,2,3,0,0,0,0,0],
vec![7,9,1,1,2,5,0,0,0,9],
vec![2,2,1,1,1,3,0,0,0,0],
vec![0,4,2,1,1,2,5,0,0,0],
vec![0,0,2,1,1,1,9,0,0,0],])]
    #[case(vec![vec![0,0,5,0,9,0,0,8,6,6],
vec![8,5,0,0,8,0,0,5,7,5],
vec![9,9,0,0,0,0,0,0,3,9],
vec![9,7,0,0,0,0,0,0,4,1],
vec![9,9,3,5,0,8,0,0,6,3],
vec![7,7,1,2,3,0,0,0,0,0],
vec![7,9,1,1,2,5,0,0,0,9],
vec![2,2,1,1,1,3,0,0,0,0],
vec![0,4,2,1,1,2,5,0,0,0],
vec![0,0,2,1,1,1,9,0,0,0],], vec![vec![2,2,6,3,0,3,1,9,7,7],
vec![0,9,2,3,0,3,1,6,9,7],
vec![0,0,3,2,2,2,1,1,5,0],
vec![0,0,4,1,1,1,1,1,6,3],
vec![0,0,7,6,1,9,1,1,7,4],
vec![0,0,5,3,4,1,1,1,2,2],
vec![0,0,4,2,3,6,1,1,2,0],
vec![5,5,3,2,2,4,1,1,2,2],
vec![1,5,3,2,2,4,7,2,1,1],
vec![1,1,3,2,2,3,0,2,1,1],])]
    #[case(vec![vec![2,2,6,3,0,3,1,9,7,7],
vec![0,9,2,3,0,3,1,6,9,7],
vec![0,0,3,2,2,2,1,1,5,0],
vec![0,0,4,1,1,1,1,1,6,3],
vec![0,0,7,6,1,9,1,1,7,4],
vec![0,0,5,3,4,1,1,1,2,2],
vec![0,0,4,2,3,6,1,1,2,0],
vec![5,5,3,2,2,4,1,1,2,2],
vec![1,5,3,2,2,4,7,2,1,1],
vec![1,1,3,2,2,3,0,2,1,1],], vec![vec![4,4,8,4,1,4,4,0,0,0],
vec![2,0,4,4,1,4,4,0,0,0],
vec![2,2,5,3,3,3,3,4,9,3],
vec![1,1,5,2,3,3,3,2,7,4],
vec![1,1,8,7,3,0,3,2,8,5],
vec![1,1,6,4,6,3,3,2,3,3],
vec![1,1,5,3,4,7,2,2,3,1],
vec![6,6,4,3,3,5,2,2,3,3],
vec![2,6,4,3,3,5,8,3,2,2],
vec![2,2,4,3,3,4,1,3,2,2],])]
    #[case(vec![vec![4,4,8,4,1,4,4,0,0,0],
vec![2,0,4,4,1,4,4,0,0,0],
vec![2,2,5,3,3,3,3,4,9,3],
vec![1,1,5,2,3,3,3,2,7,4],
vec![1,1,8,7,3,0,3,2,8,5],
vec![1,1,6,4,6,3,3,2,3,3],
vec![1,1,5,3,4,7,2,2,3,1],
vec![6,6,4,3,3,5,2,2,3,3],
vec![2,6,4,3,3,5,8,3,2,2],
vec![2,2,4,3,3,4,1,3,2,2],], vec![vec![5,5,9,5,2,5,5,1,1,1],
vec![3,1,5,5,2,5,5,2,2,2],
vec![3,3,6,4,4,4,4,6,0,5],
vec![2,2,6,3,4,4,4,4,9,6],
vec![2,2,9,8,4,1,4,3,9,6],
vec![2,2,7,5,7,4,4,3,4,4],
vec![2,2,6,4,5,8,3,3,4,2],
vec![7,7,5,4,4,6,3,3,4,4],
vec![3,7,5,4,4,6,9,4,3,3],
vec![3,3,5,4,4,5,2,4,3,3],])]
    #[case(vec![vec![5,5,9,5,2,5,5,1,1,1],
vec![3,1,5,5,2,5,5,2,2,2],
vec![3,3,6,4,4,4,4,6,0,5],
vec![2,2,6,3,4,4,4,4,9,6],
vec![2,2,9,8,4,1,4,3,9,6],
vec![2,2,7,5,7,4,4,3,4,4],
vec![2,2,6,4,5,8,3,3,4,2],
vec![7,7,5,4,4,6,3,3,4,4],
vec![3,7,5,4,4,6,9,4,3,3],
vec![3,3,5,4,4,5,2,4,3,3],], vec![vec![6,7,0,7,3,6,6,2,2,2],
vec![4,3,7,7,3,6,6,3,3,3],
vec![4,4,7,5,5,5,5,8,2,7],
vec![3,4,9,6,6,5,5,7,0,9],
vec![3,5,0,0,6,2,5,6,0,9],
vec![3,5,0,9,9,5,5,5,6,6],
vec![3,4,8,6,6,9,4,4,5,3],
vec![8,8,6,5,5,8,5,5,5,5],
vec![4,8,6,5,5,8,0,6,4,4],
vec![4,4,6,5,5,7,4,6,4,4],])]
    #[case(vec![vec![6,7,0,7,3,6,6,2,2,2],
vec![4,3,7,7,3,6,6,3,3,3],
vec![4,4,7,5,5,5,5,8,2,7],
vec![3,4,9,6,6,5,5,7,0,9],
vec![3,5,0,0,6,2,5,6,0,9],
vec![3,5,0,9,9,5,5,5,6,6],
vec![3,4,8,6,6,9,4,4,5,3],
vec![8,8,6,5,5,8,5,5,5,5],
vec![4,8,6,5,5,8,0,6,4,4],
vec![4,4,6,5,5,7,4,6,4,4],], vec![vec![7,8,1,8,4,7,7,3,3,3],
vec![5,4,8,8,4,7,7,4,4,4],
vec![5,6,9,7,6,6,6,9,4,9],
vec![4,6,0,8,7,6,6,8,3,0],
vec![4,7,3,4,9,4,6,7,3,0],
vec![4,7,4,0,0,9,7,6,8,8],
vec![6,9,0,0,0,0,7,5,6,4],
vec![0,0,0,0,0,0,9,6,6,6],
vec![8,0,0,0,0,0,4,7,5,5],
vec![6,8,0,0,0,0,7,7,5,5],])]
    #[case(vec![vec![7,8,1,8,4,7,7,3,3,3],
vec![5,4,8,8,4,7,7,4,4,4],
vec![5,6,9,7,6,6,6,9,4,9],
vec![4,6,0,8,7,6,6,8,3,0],
vec![4,7,3,4,9,4,6,7,3,0],
vec![4,7,4,0,0,9,7,6,8,8],
vec![6,9,0,0,0,0,7,5,6,4],
vec![0,0,0,0,0,0,9,6,6,6],
vec![8,0,0,0,0,0,4,7,5,5],
vec![6,8,0,0,0,0,7,7,5,5],], vec![vec![9,0,6,0,0,0,0,6,4,4],
vec![7,8,0,0,0,0,0,9,7,6],
vec![6,9,0,0,0,0,0,0,8,0],
vec![5,8,4,0,0,0,0,0,8,2],
vec![5,8,5,8,0,0,0,0,9,3],
vec![6,9,6,2,4,0,0,0,0,0],
vec![8,0,2,1,2,5,0,0,0,9],
vec![2,2,2,1,1,3,0,0,0,9],
vec![9,1,1,1,1,2,8,0,9,7],
vec![7,9,1,1,1,1,9,9,7,6],])]
    #[case(vec![vec![9,0,6,0,0,0,0,6,4,4],
vec![7,8,0,0,0,0,0,9,7,6],
vec![6,9,0,0,0,0,0,0,8,0],
vec![5,8,4,0,0,0,0,0,8,2],
vec![5,8,5,8,0,0,0,0,9,3],
vec![6,9,6,2,4,0,0,0,0,0],
vec![8,0,2,1,2,5,0,0,0,9],
vec![2,2,2,1,1,3,0,0,0,9],
vec![9,1,1,1,1,2,8,0,9,7],
vec![7,9,1,1,1,1,9,9,7,6],], vec![vec![0,4,8,1,1,1,2,9,7,6],
vec![0,0,3,1,1,1,2,0,0,9],
vec![0,0,4,1,1,1,2,5,0,4],
vec![0,0,8,1,1,1,1,4,0,6],
vec![0,0,9,9,1,1,1,3,0,6],
vec![0,0,9,3,5,1,1,2,3,3],
vec![0,4,4,2,3,6,1,1,3,0],
vec![5,5,3,2,2,5,2,3,5,0],
vec![0,5,3,2,2,5,0,6,0,0],
vec![0,0,3,2,2,4,0,0,0,0],])]
    fn test_example_input_population(#[case] before: Vec<Vec<u8>>, #[case] after: Vec<Vec<u8>>) {
        assert_eq!(step(before).0, after);
    }

    #[rstest]
    #[case(vec![vec![5,4,8,3,1,4,3,2,2,3],
        vec![2,7,4,5,8,5,4,7,1,1],
        vec![5,2,6,4,5,5,6,1,7,3],
        vec![6,1,4,1,3,3,6,1,4,6],
        vec![6,3,5,7,3,8,5,4,7,8],
        vec![4,1,6,7,5,2,4,6,4,5],
        vec![2,1,7,6,8,4,1,7,2,1],
        vec![6,8,8,2,8,8,1,1,3,4],
        vec![4,8,4,6,8,4,8,5,5,4],
        vec![5,2,8,3,7,5,1,5,2,6],], 100, 1656)]
    fn test_example_input_n_flashes(
        #[case] start: Vec<Vec<u8>>,
        #[case] n_steps: u32,
        #[case] expected_flashes: u32,
    ) {
        assert_eq!(steps(start, n_steps), expected_flashes);
    }
    #[rstest]
    #[case(vec![vec![5,4,8,3,1,4,3,2,2,3],
        vec![2,7,4,5,8,5,4,7,1,1],
        vec![5,2,6,4,5,5,6,1,7,3],
        vec![6,1,4,1,3,3,6,1,4,6],
        vec![6,3,5,7,3,8,5,4,7,8],
        vec![4,1,6,7,5,2,4,6,4,5],
        vec![2,1,7,6,8,4,1,7,2,1],
        vec![6,8,8,2,8,8,1,1,3,4],
        vec![4,8,4,6,8,4,8,5,5,4],
        vec![5,2,8,3,7,5,1,5,2,6],], 195)]
    fn test_example_input_all_flash(#[case] start: Vec<Vec<u8>>, #[case] n_steps: u32) {
        assert_eq!(steps_until_all_flash(start), n_steps);
    }

    #[rstest]
    #[case((0, 0), (3, 3), vec![(0, 0), (1, 0), (0, 1), (1, 1)])]
    #[case((0, 1), (3, 3), vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2)])]
    #[case((0, 2), (3, 3), vec![(0, 2), (1, 2), (0, 1), (1, 1)])]
    #[case((1, 0), (3, 3), vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1),])]
    #[case((1, 1), (3, 3), vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1), (0, 2), (1, 2), (2, 2)])]
    #[case((1, 2), (3, 3), vec![(0, 2), (1, 2), (2, 2), (0, 1), (1, 1), (2, 1)])]
    #[case((2, 0), (3, 3), vec![(2, 0), (2, 1), (1, 0), (1, 1)])]
    #[case((2, 1), (3, 3), vec![(2, 0), (2, 1), (2, 2), (1, 0), (1, 1), (1, 2)])]
    #[case((2, 2), (3, 3), vec![(2, 2), (1, 2), (2, 1), (1, 1)])]
    fn test_retrieve_neighbours(
        #[case] center: (usize, usize),
        #[case] size: (usize, usize),
        #[case] mut expected: Vec<(usize, usize)>,
    ) {
        let mut actual = retrieve_safe_neighbours(center.0, center.1, size.0, size.1);
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_actual_input() {
        println!("Hello, world!");
        let input = vec![
            vec![7, 7, 7, 7, 8, 3, 8, 3, 5, 3],
            vec![2, 2, 1, 7, 2, 7, 2, 4, 7, 8],
            vec![3, 3, 5, 5, 3, 1, 8, 6, 4, 5],
            vec![2, 2, 4, 2, 6, 1, 8, 1, 1, 3],
            vec![7, 1, 8, 2, 4, 6, 8, 6, 6, 6],
            vec![5, 4, 4, 1, 6, 4, 1, 1, 1, 1],
            vec![4, 7, 7, 3, 8, 6, 2, 3, 6, 4],
            vec![5, 7, 1, 7, 1, 2, 5, 5, 2, 1],
            vec![7, 5, 4, 2, 1, 2, 7, 7, 2, 1],
            vec![4, 5, 7, 6, 6, 7, 8, 3, 4, 1],
        ];

        let flashes = steps(input.clone(), 100);

        let steps_until_all = steps_until_all_flash(input);

        assert_eq!(flashes, 1721);
        assert_eq!(steps_until_all, 298);
    }

    #[bench]
    fn speed_oac_part_1(b: &mut Bencher) {
        let input = vec![
            vec![7, 7, 7, 7, 8, 3, 8, 3, 5, 3],
            vec![2, 2, 1, 7, 2, 7, 2, 4, 7, 8],
            vec![3, 3, 5, 5, 3, 1, 8, 6, 4, 5],
            vec![2, 2, 4, 2, 6, 1, 8, 1, 1, 3],
            vec![7, 1, 8, 2, 4, 6, 8, 6, 6, 6],
            vec![5, 4, 4, 1, 6, 4, 1, 1, 1, 1],
            vec![4, 7, 7, 3, 8, 6, 2, 3, 6, 4],
            vec![5, 7, 1, 7, 1, 2, 5, 5, 2, 1],
            vec![7, 5, 4, 2, 1, 2, 7, 7, 2, 1],
            vec![4, 5, 7, 6, 6, 7, 8, 3, 4, 1],
        ];

        b.iter(|| steps(input.clone(), 100));
    }

    #[bench]
    fn speed_oac_part_2(b: &mut Bencher) {
        let input = vec![
            vec![7, 7, 7, 7, 8, 3, 8, 3, 5, 3],
            vec![2, 2, 1, 7, 2, 7, 2, 4, 7, 8],
            vec![3, 3, 5, 5, 3, 1, 8, 6, 4, 5],
            vec![2, 2, 4, 2, 6, 1, 8, 1, 1, 3],
            vec![7, 1, 8, 2, 4, 6, 8, 6, 6, 6],
            vec![5, 4, 4, 1, 6, 4, 1, 1, 1, 1],
            vec![4, 7, 7, 3, 8, 6, 2, 3, 6, 4],
            vec![5, 7, 1, 7, 1, 2, 5, 5, 2, 1],
            vec![7, 5, 4, 2, 1, 2, 7, 7, 2, 1],
            vec![4, 5, 7, 6, 6, 7, 8, 3, 4, 1],
        ];

        b.iter(|| steps_until_all_flash(input.clone()));
    }
}
