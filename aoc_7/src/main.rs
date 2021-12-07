use std::fs::File;
use std::io::Read;

fn main() {
    let input = read_input();
    println!("Part 1: {:?}", aoc_7(&input, calculate_alignment_cost));
    println!("Part 2: {:?}", aoc_7(&input, calculate_alignment_cost_exp))
}

fn read_input() -> Vec<i32> {
    let mut file = File::open("src/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    return contents
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
}

fn calculate_alignment_cost(crab_positions: &Vec<i32>, target: i32) -> i32 {
    return crab_positions
        .iter()
        .map(|p| (p - target).abs())
        .sum::<i32>();
}

fn calculate_alignment_cost_exp(crab_positions: &Vec<i32>, target: i32) -> i32 {
    return crab_positions
        .iter()
        .map(|p| {
            let n = (p - target).abs();
            n * (n + 1) / 2
        })
        .sum::<i32>();
}

fn aoc_7(crab_positions: &Vec<i32>, cost_fn: fn(&Vec<i32>, i32) -> i32) -> i32 {
    let min_pos = *crab_positions.iter().min().unwrap();
    let max_pos = *crab_positions.iter().max().unwrap();
    let mut min_cost: i32 = i32::MAX;
    for i in min_pos..max_pos {
        let cost = cost_fn(&crab_positions, i);
        if cost < min_cost {
            min_cost = cost;
        }
    }

    return min_cost;
}

#[test]
fn test_example_input() {
    let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    assert_eq!(aoc_7(&input, calculate_alignment_cost), 37);
}

#[test]
fn test_alignment_calculation() {
    let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    assert_eq!(calculate_alignment_cost(&input, 2), 37);
    assert_eq!(calculate_alignment_cost(&input, 1), 41);
    assert_eq!(calculate_alignment_cost(&input, 3), 39);
    assert_eq!(calculate_alignment_cost(&input, 10), 71);
}

#[test]
fn test_alignment_calculation_exp() {
    let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    assert_eq!(calculate_alignment_cost_exp(&input, 5), 168);
    assert_eq!(calculate_alignment_cost_exp(&input, 2), 206);
}

#[test]
fn test_alignment_calculation_exp_small() {
    let input = vec![0];

    assert_eq!(calculate_alignment_cost_exp(&input, 0), 0);
    assert_eq!(calculate_alignment_cost_exp(&input, 1), 1);
    assert_eq!(calculate_alignment_cost_exp(&input, 2), 3);
    assert_eq!(calculate_alignment_cost_exp(&input, 3), 6);
    assert_eq!(calculate_alignment_cost_exp(&input, 4), 10);
}
