#![feature(destructuring_assignment)]
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufReader, Read};

pub fn read_file(file: &str) -> Vec<Vec<u16>> {
    let input = File::open(file).unwrap();

    let mut contents = String::new();
    BufReader::new(input).read_to_string(&mut contents).unwrap();

    return read_from_str(&contents);
}

pub fn aoc_15_comp(input: &str) -> (u16, u16) {
    let vec_map = read_from_str(input);
    let map_part_1 = Map::new(vec_map.clone(), 1);
    let part_1 = calculate_path_cost(map_part_1).unwrap();
    let map_part_2 = Map::new(vec_map.clone(), 5);
    let part_2 = calculate_path_cost(map_part_2).unwrap();
    return (part_1, part_2);
}

fn read_from_str(input: &str) -> Vec<Vec<u16>> {
    return input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u16)
                .collect()
        })
        .collect();
}

pub fn aoc_15(file: &str, multiplier: u16) -> u16 {
    let input = read_file(file);
    let map_part_1 = Map::new(input, multiplier);
    return calculate_path_cost(map_part_1).unwrap();
}

pub struct Map {
    risk_map: Vec<Vec<u16>>,
    multiplier: u16,
}

impl Map {
    pub fn new(risk_map: Vec<Vec<u16>>, multiplier: u16) -> Map {
        return Map {
            risk_map,
            multiplier,
        };
    }

    fn get(&self, index: (u16, u16)) -> u16 {
        let i = index.0 % self.risk_map.len() as u16;
        let i_risk = index.0 / self.risk_map.len() as u16;

        let j = index.1 % self.risk_map[0].len() as u16;
        let j_risk = index.1 / self.risk_map[0].len() as u16;

        let risk = &self.risk_map[i as usize][j as usize];

        let total_risk = ((risk + i_risk + j_risk - 1) % 9) + 1;

        return total_risk;
    }

    fn size(&self) -> (usize, usize) {
        return (
            &self.risk_map.len() * self.multiplier as usize,
            &self.risk_map[0].len() * self.multiplier as usize,
        );
    }

    fn get_goal(&self) -> (u16, u16) {
        return (
            (self.risk_map.len() as u16 * self.multiplier) - 1,
            (self.risk_map[0].len() as u16 * self.multiplier) - 1,
        );
    }
}

pub fn calculate_path_cost(matrix: Map) -> Option<u16> {
    let start = (0, 0);
    let goal = matrix.get_goal();
    return a_star(start, goal, matrix);
}

fn a_star(start: (u16, u16), goal: (u16, u16), matrix: Map) -> Option<u16> {
    let f_start = heuristic(start, goal);
    let mut open_set = BinaryHeap::from([(Reverse(f_start), start)]);

    let size = matrix.size();

    let mut g_scores = vec![vec![u16::MAX; size.1]; size.0];
    let mut f_scores = vec![vec![u16::MAX; size.1]; size.0];

    set(&mut g_scores, &start, 0);
    set(&mut f_scores, &start, f_start);

    let mut current;
    while !open_set.is_empty() {
        current = open_set.pop().unwrap().1;

        if current == goal {
            return Some(get(&g_scores, &current));
        }

        for neighbour in get_safe_neighbours(current, size.0 as u16, size.1 as u16) {
            let g_score = get(&g_scores, &current) + matrix.get(neighbour);
            if g_score < get(&g_scores, &neighbour) {
                set(&mut g_scores, &neighbour, g_score);
                let new_f_score = g_score + heuristic(neighbour, goal);
                set(&mut f_scores, &neighbour, new_f_score);
                open_set.push((Reverse(new_f_score), neighbour));
            }
        }
    }

    return None;
}

fn get<T: Into<usize> + Copy>(matrix: &Vec<Vec<u16>>, point: &(T, T)) -> u16 {
    matrix[point.0.into()][point.1.into()]
}

fn set<T: Into<usize> + Copy, V>(matrix: &mut Vec<Vec<V>>, point: &(T, T), value: V) {
    matrix[point.0.into()][point.1.into()] = value;
}

fn get_safe_neighbours(current: (u16, u16), len_x: u16, len_y: u16) -> Vec<(u16, u16)> {
    let mut neighbours = vec![];
    let (i, j) = current.clone();

    if i + 1 < len_x {
        neighbours.push((i + 1, j));
    }
    if 0 <= i as i16 - 1 {
        neighbours.push((i - 1, j));
    }
    if j + 1 < len_y {
        neighbours.push((i, j + 1));
    }
    if 0 <= j as i16 - 1 {
        neighbours.push((i, j - 1));
    }

    return neighbours;
}

fn heuristic(current: (u16, u16), goal: (u16, u16)) -> u16 {
    return (goal.0 - current.0 + goal.1 - current.1) as u16;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_path_cost_example_input() {
        let input = vec![
            vec![1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
            vec![1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
            vec![2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
            vec![3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
            vec![7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
            vec![1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
            vec![1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
            vec![3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
            vec![1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
            vec![2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
        ];

        let risk_map = Map {
            risk_map: input,
            multiplier: 1,
        };

        assert_eq!(calculate_path_cost(risk_map), Some(40));
    }

    #[test]
    fn test_find_path_cost_example_input_part_2() {
        let input = vec![
            vec![1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
            vec![1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
            vec![2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
            vec![3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
            vec![7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
            vec![1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
            vec![1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
            vec![3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
            vec![1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
            vec![2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
        ];

        let risk_map = Map {
            risk_map: input,
            multiplier: 5,
        };

        assert_eq!(calculate_path_cost(risk_map), Some(315));
    }
}
