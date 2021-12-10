#![feature(test)]
#![feature(string_remove_matches)]
use stats::median;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    println!("Hello, world!");

    let input_str = read_file(String::from("src/input"));
    let (part_1, part_2) = aoc_10(&input_str);

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}

fn read_file(file: String) -> String {
    let input = File::open(file).unwrap();

    let mut contents = String::new();
    BufReader::new(input).read_to_string(&mut contents).unwrap();

    return contents;
}

fn aoc_10(input_string: &String) -> (u32, u32) {
    let score_conv = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    let score_part_1 = input_string
        .lines()
        .filter_map(|line| filter_corrupt(line))
        .filter_map(|c| score_conv.get(&c))
        .sum();

    let score_part_2 = median(
        input_string
            .lines()
            .filter_map(|line| fix_incomplete(line))
            .map(|comp| calculate_score_part_2(comp)),
    )
    .unwrap() as u32;

    return (score_part_1, score_part_2);
}

fn calculate_score_part_2(auto_complete: Vec<char>) -> u64 {
    let score_conv = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    let mut score = 0;
    for char_ in auto_complete {
        score *= 5;
        score += score_conv.get(&char_).unwrap();
    }
    return score;
}

fn fix_incomplete(line: &str) -> Option<Vec<char>> {
    let open = HashSet::from(['[', '(', '{', '<']);
    let converter_oc = HashMap::from([('[', ']'), ('(', ')'), ('{', '}'), ('<', '>')]);

    let mut open_chars = vec![];
    for char_ in line.chars() {
        if open.contains(&char_) {
            open_chars.push(char_)
        } else {
            let last_open = open_chars.pop().unwrap();
            let expected = converter_oc.get(&last_open).unwrap();
            if expected != &char_ {
                return None;
            }
        }
    }

    return Some(
        open_chars
            .iter()
            .rev()
            .map(|open| *converter_oc.get(open).unwrap())
            .collect::<Vec<char>>(),
    );
}

fn filter_corrupt(line: &str) -> Option<char> {
    let open = HashSet::from(['[', '(', '{', '<']);
    let converter_oc = HashMap::from([('[', ']'), ('(', ')'), ('{', '}'), ('<', '>')]);

    let mut corrupt = None;
    let mut open_chars = vec![];

    for char_ in line.chars() {
        if open.contains(&char_) {
            open_chars.push(char_)
        } else {
            let last_open = open_chars.pop().unwrap();
            let expected = converter_oc.get(&last_open).unwrap();
            if expected != &char_ {
                corrupt = Some(char_);
                break;
            }
        }
    }

    return corrupt;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    extern crate test;
    use test::Bencher;

    #[rstest]
    #[case::sad("{([(<{}[<>[]}>{[]{[(<()>", Some('}'))]
    #[case::sad("[[<[([]))<([[{}[[()]]]", Some(')'))]
    #[case::sad("[{[{({}]{}}([{[{{{}}([]", Some(']'))]
    #[case::sad("[<(<(<(<{}))><([]([]()", Some(')'))]
    #[case::sad("<{([([[(<>()){}]>(<<{{", Some('>'))]
    #[case::happy("[({(<(())[]>[[{[]{<()<>>", None)]
    #[case::happy("[(()[<>])]({[<{<<[]>>(", None)]
    fn test_filter_corrupt(#[case] input: &str, #[case] corrupt_character: Option<char>) {
        assert_eq!(filter_corrupt(input), corrupt_character);
    }

    #[rstest]
    #[case("[({(<(())[]>[[{[]{<()<>>", "}}]])})]")]
    #[case("[(()[<>])]({[<{<<[]>>(", ")}>]})")]
    #[case("(((({<>}<{<{<>}{[]{[]{}", "}}>}>))))")]
    #[case("{<[[]]>}<{[{[{[]{()[[[]", "]]}}]}]}>")]
    #[case("<{([{{}}[<[[[<>{}]]]>[]]", "])}>")]
    fn test_fix_incomplete(#[case] input: &str, #[case] completion: &str) {
        assert_eq!(
            fix_incomplete(input),
            Some(completion.chars().collect::<Vec<char>>())
        );
    }

    #[rstest]
    #[case("}}]])})]", 288957)]
    #[case(")}>]})", 5566)]
    #[case("}}>}>))))", 1480781)]
    #[case("]]}}]}]}>", 995444)]
    #[case("])}>", 294)]
    fn test_calculate_score_part_2(#[case] input: &str, #[case] score: u64) {
        assert_eq!(
            calculate_score_part_2(input.chars().collect::<Vec<char>>()),
            score
        )
    }

    #[test]
    fn test_example_input() {
        let input = String::from(
            "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]",
        );
        let (part_1, part_2) = aoc_10(&input);
        assert_eq!(part_1, 26397);
        assert_eq!(part_2, 288957);
    }

    #[test]
    fn test_actual_input() {
        let input_str = read_file(String::from("src/input"));
        let (part_1, part_2) = aoc_10(&input_str);
        assert_eq!(part_1, 374061);
        assert_eq!(part_2, 2116639949);
    }

    #[bench]
    fn bench_day_10(b: &mut Bencher) {
        let input = read_file(String::from("src/input"));

        b.iter(|| aoc_10(&input));
    }
}
