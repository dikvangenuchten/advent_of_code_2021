use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() {
    println!("Hello, world!");
    println!("Part 1: {:?}", oac_9(String::from("src/input")));
}

fn oac_9(file: String) -> u32 {
    let input = File::open(file).unwrap();

    let mut contents = String::new();
    BufReader::new(input).read_to_string(&mut contents).unwrap();

    let input_matrix = parse_input(contents);
    let lowest_points = find_lowest_poinst(input_matrix);
    return calculate_risk(lowest_points);
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

fn find_lowest_poinst(input_matrix: Vec<Vec<u8>>) -> Vec<u8> {
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
            lowest_points.push(center);
        }
    }
    return lowest_points;
}

fn calculate_risk(points: Vec<u8>) -> u32 {
    return points.iter().fold(0, |a, &b| a + 1 + b as u32);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_example_input() {
        let input = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];

        let out = find_lowest_poinst(input);

        assert_eq!(out, vec![1, 0, 5, 5]);
    }

    #[rstest]
    #[case(vec![vec![2, 1, 9], vec![3, 9, 8], vec![9, 8, 5]], vec![1, 5])]
    #[case(vec![vec![2, 1, 9], vec![3, 0, 8], vec![9, 8, 5]], vec![0, 5])]
    #[case(vec![vec![1, 2, 2], vec![2, 2, 2], vec![2, 2, 2]], vec![1])]
    #[case(vec![vec![2, 1, 2], vec![2, 2, 2], vec![2, 2, 2]], vec![1])]
    #[case(vec![vec![2, 2, 1], vec![2, 2, 2], vec![2, 2, 2]], vec![1])]
    #[case(vec![vec![2, 2, 2], vec![1, 2, 2], vec![2, 2, 2]], vec![1])]
    #[case(vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]], vec![1])]
    #[case(vec![vec![2, 2, 2], vec![2, 2, 1], vec![2, 2, 2]], vec![1])]
    #[case(vec![vec![2, 2, 2], vec![2, 2, 2], vec![1, 2, 2]], vec![1])]
    #[case(vec![vec![2, 2, 2], vec![2, 2, 2], vec![2, 1, 2]], vec![1])]
    #[case(vec![vec![2, 2, 2], vec![2, 2, 2], vec![2, 2, 1]], vec![1])]
    fn test_example_easy(#[case] input: Vec<Vec<u8>>, #[case] ex_out: Vec<u8>) {
        let out = find_lowest_poinst(input);

        assert_eq!(out, ex_out);
    }

    #[test]
    fn test_parse_input() {
        let input = String::from("000\n000\n000");
        let out = parse_input(input);

        assert_eq!(out, vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]);
    }
}
