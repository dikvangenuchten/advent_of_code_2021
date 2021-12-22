use itertools::Itertools;
use std::collections::HashMap;
use std::str::ParseBoolError;
use std::{iter::Repeat, str::FromStr, string::ParseError};

use std::fs::File;
use std::io::{BufReader, Read};
pub fn read_file(file: &str) -> String {
    let input = File::open(file).unwrap();

    let mut contents = String::new();
    BufReader::new(input).read_to_string(&mut contents).unwrap();

    return contents;
}

pub fn aoc_21_comp(input_str: &str) -> (u32, u64) {
    return (day_21_part_1(input_str), day_21_part_2(input_str));
}

pub fn day_21_part_1(input_str: &str) -> u32 {
    let (player1_str, player2_str) = input_str.split_once("\n").unwrap();

    let mut player1 = Player::from_str(player1_str).unwrap();
    let mut player2 = Player::from_str(player2_str).unwrap();
    let mut die = DeterministicDice::new();
    return game(&mut die, &mut player1, &mut player2);
}

pub fn day_21_part_2(input_str: &str) -> u64 {
    let initial_state = GameState::from_str(&input_str).unwrap();
    return game_iteration_part_2(initial_state);
}

fn game(die: &mut impl Throw, player1: &mut Player, player2: &mut Player) -> u32 {
    let mut die_iter = die.iter(3);

    let winner;
    loop {
        if player1.step(die_iter.next().unwrap()) {
            winner = player1.id;
            break;
        };
        if player2.step(die_iter.next().unwrap()) {
            winner = player2.id;
            break;
        };
    }

    if winner == player1.id {
        return player2.score as u32 * die.get_count() as u32;
    } else {
        return player1.score as u32 * die.get_count() as u32;
    }
}

struct DeterministicDice {
    current: u16,
    count: usize,
}

impl DeterministicDice {
    fn new() -> Self {
        return Self {
            current: 1,
            count: 0,
        };
    }
}

trait Throw {
    fn throw(self: &mut Self) -> u16;
    fn iter(&mut self, n: u16) -> DiceIter<'_, Self>
    where
        Self: Sized;
    fn get_count(&self) -> usize;
}

impl Throw for DeterministicDice {
    fn throw(self: &mut Self) -> u16 {
        let throw = self.current;
        self.current += 1;
        self.count += 1;
        if self.current > 100 {
            self.current = 1;
        }
        return throw;
    }

    fn iter(&mut self, n: u16) -> DiceIter<'_, DeterministicDice> {
        return DiceIter { dice: self, n };
    }

    fn get_count(&self) -> usize {
        return self.count;
    }
}

struct DiceIter<'a, Dice>
where
    Dice: Throw,
{
    dice: &'a mut Dice,
    n: u16,
}

impl<'a, Dice> Iterator for DiceIter<'a, Dice>
where
    Dice: Throw,
{
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        let mut count = 0;
        for _ in 0..self.n {
            count += self.dice.throw();
        }
        return Some(count);
    }
}

struct Player {
    id: u8,
    position: u16,
    score: u16,
}

impl FromStr for Player {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, position) = s.strip_prefix("Player ").unwrap().split_once(" ").unwrap();
        println!("id: {id}\nposition: {position}");
        let id = id.parse::<u8>().expect("Unable to find id");
        let position = position
            .strip_prefix("starting position: ")
            .unwrap()
            .trim()
            .parse::<u16>()
            .expect("Unable to find position");

        return Ok(Player::new(id, position));
    }
}

impl Player {
    fn new(id: u8, position: u16) -> Player {
        return Player {
            id,
            position,
            score: 0,
        };
    }

    fn step(self: &mut Self, steps: u16) -> bool {
        self.position = ((self.position + steps - 1) % 10) + 1;
        self.score += self.position;
        return self.score >= 1000;
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct GameState {
    position1: u16,
    position2: u16,
    score1: u8,
    score2: u8,
    throws: u8,
    player1_turn: bool,
}

impl GameState {
    fn quantum_step(self: Self) -> Vec<(Self, u8)> {
        const QUANTUM_THROWS: [(u8, u8); 7] =
            [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

        let mut new_states = vec![];
        for (i, weight) in QUANTUM_THROWS {
            new_states.push((self.clone().step(i), weight))
        }
        return new_states;
    }

    fn step(mut self, steps: u8) -> Self {
        if self.player1_turn {
            self.position1 = ((self.position1 - 1 + steps as u16) % 10) + 1;
            self.score1 += self.position1 as u8;
        } else {
            self.position2 = ((self.position2 - 1 + steps as u16) % 10) + 1;
            self.score2 += self.position2 as u8;
        }
        self.player1_turn = !self.player1_turn;
        self.throws += 1;
        return self;
    }

    fn get_winner(self: &Self) -> Option<u8> {
        if self.score1 >= 21 {
            return Some(1);
        } else if self.score2 >= 21 {
            return Some(2);
        }
        return None;
    }
}

impl FromStr for GameState {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (player1_str, player2_str) = s.trim().split_once("\n").unwrap();

        fn retrieve_id_position(input: &str) -> (u8, u16) {
            let (id, position) = input
                .strip_prefix("Player ")
                .unwrap()
                .split_once(" ")
                .unwrap();
            let id = id.parse::<u8>().expect("Unable to find id");
            let position = position
                .strip_prefix("starting position: ")
                .unwrap()
                .trim()
                .parse::<u16>()
                .expect("Unable to find position");
            return (id, position);
        }

        let (_, p1) = retrieve_id_position(player1_str);
        let (_, p2) = retrieve_id_position(player2_str);
        return Ok(GameState {
            position1: p1,
            position2: p2,
            score1: 0,
            score2: 0,
            throws: 0,
            player1_turn: true,
        });
    }
}

fn game_iteration_part_2(initial_state: GameState) -> u64 {
    let mut states: HashMap<GameState, u64> = HashMap::from([(initial_state, 1)]);

    let mut p1_games = 0;
    let mut p2_games = 0;

    while !states.is_empty() {
        let mut new_states = HashMap::<GameState, u64>::new();
        for (state, weight) in states.drain() {
            for (new_state, multiplier) in state.quantum_step() {
                let weight = weight * multiplier as u64;
                if let Some(winner) = new_state.get_winner() {
                    if winner == 1 {
                        p1_games += weight;
                    } else {
                        p2_games += weight;
                    }
                } else {
                    // println!("weight: {}", weight);
                    *new_states.entry(new_state).or_insert(0) += weight;
                }
            }
        }
        states = new_states;
    }

    println!("{}, {}", p1_games, p2_games);

    return p1_games.max(p2_games);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_dice() {
        let mut die = DeterministicDice::new();

        let range: Vec<u16> = (1..101).cycle().take(1000).collect();
        let die_throws: Vec<u16> = die.iter(1).take(1000).collect();

        assert_eq!(range, die_throws);
        assert_eq!(die.count, 1_000);
    }

    #[rstest]
    #[case("src/example_input", 745 * 993)]
    #[case("src/input", 908091)]
    fn test_part_1(#[case] input_file: &str, #[case] expected_score: u32) {
        let input_str = read_file(input_file);
        assert_eq!(day_21_part_1(&input_str), expected_score);
    }

    #[rstest]
    #[case("src/example_input", 444356092776315)]
    #[case("src/input", 190897246590017)]
    fn test_part_2(#[case] input_file: &str, #[case] expected_score: u64) {
        let input_str = read_file(input_file);
        let initial_state = GameState::from_str(&input_str).unwrap();
        assert_eq!(game_iteration_part_2(initial_state), expected_score);
    }
}
