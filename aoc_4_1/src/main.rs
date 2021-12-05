use std::fs::File;
use std::io::{BufRead, BufReader, Read};
fn main() {
    println!("Day 4 Part 1: {:?}", &aoc_4_1());
}

struct BingoBoard {
    board: Vec<Vec<u8>>,
    score: u32,
    unmarked: Vec<u8>,
}

fn construct_bingoboard(board: Vec<Vec<u8>>) -> BingoBoard {
    assert!(!board.is_empty());
    return BingoBoard {
        board: board.clone(),
        score: 0,
        unmarked: board
            .iter()
            .flatten()
            .map(|x| x.clone())
            .collect::<Vec<u8>>(),
    };
}

impl BingoBoard {
    fn cross(&mut self, draw: u8) -> bool {
        self.unmarked.retain(|x| x != &draw);
        if self.is_complete() {
            self.calculate_score(draw);
            return true;
        }
        return false;
    }

    fn is_complete(&mut self) -> bool {
        return self.has_complete_rows() || self.has_complete_collumns();
    }

    fn has_complete_rows(&mut self) -> bool {
        for row in &self.board {
            if !row.iter().any(|x| self.unmarked.contains(x)) {
                return true;
            }
        }
        return false;
    }

    fn has_complete_collumns(&mut self) -> bool {
        for i in 0..self.board[0].len() {
            let mut collumn_complete = true;
            for j in 0..self.board.len() {
                if self.unmarked.contains(&self.board[j][i]) {
                    collumn_complete = false;
                    break;
                }
            }
            if collumn_complete {
                return true;
            }
        }
        return false;
    }

    fn calculate_score(&mut self, draw: u8) {
        let score: u32 = self.unmarked.iter().map(|&x| x as u32).sum();
        self.score = score * draw as u32;
    }
}

fn aoc_4_1() -> u32 {
    let file = File::open("src/input").unwrap();
    let draws_str = &mut String::new();
    let mut reader = BufReader::new(file);
    reader.read_line(draws_str).unwrap();
    let draws = draws_str
        .trim()
        .split(",")
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    println!("Draws: {:?}", draws);

    // let mut board_str;

    let board_str = &mut String::from("");
    reader.read_to_string(board_str).unwrap();

    let mut boards = split_boards(board_str.to_string())
        .iter()
        .filter_map(|board| parse_board(&board))
        .collect::<Vec<BingoBoard>>();

    let mut full = false;
    let mut max_score = 0;
    for draw in draws {
        print!("{:?}, ", draw);
        for board in boards.iter_mut() {
            if board.cross(draw) {
                full = true;
                if board.score > max_score {
                    max_score = board.score;
                    println! {"\nFull: {:?}", board.board}
                }
            }
        }
        if full {
            break;
        }
    }

    return max_score;
}

fn split_boards(boards_str: String) -> Vec<String> {
    boards_str
        .split("\n\n")
        .map(|s| s.into())
        .collect::<Vec<String>>()
}

fn parse_board(board_str: &String) -> Option<BingoBoard> {
    let board = board_str
        .lines()
        .filter(|s| !s.trim().is_empty())
        .map(|row| {
            row.split(" ")
                .filter(|s| !s.trim().is_empty())
                .map(|x| x.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    if board.is_empty() {
        return None;
    }
    return Some(construct_bingoboard(board));
}

#[test]
fn test_parse_board() {
    let input = String::from(
        "
22 59  7 10  6
33 36 96 55 23
13 85 18 29 28
75 46 83 73 58
34 40 87 56 98
",
    );

    let out = parse_board(&input);

    let expected_out = construct_bingoboard(vec![
        vec![22, 59, 7, 10, 6],
        vec![33, 36, 96, 55, 23],
        vec![13, 85, 18, 29, 28],
        vec![75, 46, 83, 73, 58],
        vec![34, 40, 87, 56, 98],
    ]);

    match out {
        Some(out) => assert_eq!(out.board, expected_out.board),
        None => assert!(false),
    }
}

#[test]
fn test_split_boards() {
    let input = String::from(
        "
22 59  7 10  6
33 36 96 55 23
13 85 18 29 28
75 46 83 73 58
34 40 87 56 98

73 96 47 14 10
28 11 79 84 20
74 30  0 59 71
80 93 42 22 17
44  2 81 29 15
",
    );

    let out = split_boards(input);

    let expected_out = vec![
        "
22 59  7 10  6
33 36 96 55 23
13 85 18 29 28
75 46 83 73 58
34 40 87 56 98",
        "
73 96 47 14 10
28 11 79 84 20
74 30  0 59 71
80 93 42 22 17
44  2 81 29 15
",
    ];

    assert_eq!(out.len(), expected_out.len());
}

#[test]
fn split_and_create() {
    let input = String::from(
        "
22 59  7 10  6
33 36 96 55 23
13 85 18 29 28
75 46 83 73 58
34 40 87 56 98

22 59  7 10  6
33 36 96 55 23
13 85 18 29 28
75 46 83 73 58
34 40 87 56 2
",
    );
    let boards = split_boards(input)
        .iter()
        .filter_map(|board| parse_board(&board))
        .collect::<Vec<BingoBoard>>();

    let bingoboard_1 = construct_bingoboard(vec![
        vec![22, 59, 7, 10, 6],
        vec![33, 36, 96, 55, 23],
        vec![13, 85, 18, 29, 28],
        vec![75, 46, 83, 73, 58],
        vec![34, 40, 87, 56, 98],
    ]);
    assert_eq!(boards[0].board, bingoboard_1.board);

    let bingoboard_2 = construct_bingoboard(vec![
        vec![22, 59, 7, 10, 6],
        vec![33, 36, 96, 55, 23],
        vec![13, 85, 18, 29, 28],
        vec![75, 46, 83, 73, 58],
        vec![34, 40, 87, 56, 2],
    ]);
    assert_eq!(boards[1].board, bingoboard_2.board);
}

#[test]
fn test_row_complete() {
    let mut bingoboard = construct_bingoboard(vec![
        vec![22, 59, 7, 10, 6],
        vec![5, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
    ]);

    assert!(!bingoboard.cross(22));
    assert_eq!(bingoboard.score, 0);
    assert!(!bingoboard.cross(59));
    assert_eq!(bingoboard.score, 0);
    assert!(!bingoboard.cross(7));
    assert_eq!(bingoboard.score, 0);
    assert!(!bingoboard.cross(10));
    assert_eq!(bingoboard.score, 0);
    assert!(bingoboard.cross(6));
    assert_eq!(bingoboard.score, 36);
}

#[test]
fn test_collumn_complete() {
    let mut bingoboard = construct_bingoboard(vec![
        vec![22, 1, 0, 0, 0],
        vec![33, 0, 0, 0, 0],
        vec![13, 0, 0, 0, 0],
        vec![75, 0, 0, 0, 0],
        vec![34, 0, 0, 0, 0],
    ]);

    assert!(!bingoboard.cross(22));
    assert_eq!(bingoboard.score, 0);
    assert!(!bingoboard.cross(33));
    assert_eq!(bingoboard.score, 0);
    assert!(!bingoboard.cross(13));
    assert_eq!(bingoboard.score, 0);
    assert!(!bingoboard.cross(75));
    assert_eq!(bingoboard.score, 0);
    assert!(bingoboard.cross(34));
    assert_eq!(bingoboard.score, 34);
}
