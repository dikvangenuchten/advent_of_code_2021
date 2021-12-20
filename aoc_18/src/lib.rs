use std::fmt::Debug;
use std::{ops::Add, str::FromStr, string::ParseError};

pub fn parse_input_str(input: &str) -> Vec<SnailFishNumber> {
    return input
        .lines()
        .map(|line| SnailFishNumber::from_str(line).unwrap())
        .collect();
}

pub fn aoc_18_part_1(input: &str) -> u32 {
    let numbers = parse_input_str(input);

    return numbers
        .into_iter()
        .reduce(|a, b| a + b)
        .unwrap()
        .magnitude();
}

pub fn aoc_18_part_2(input: &str) -> u32 {
    let numbers_1 = parse_input_str(input);
    let numbers_2 = parse_input_str(input);

    let mut max_magnitude = 0;

    for lhs in numbers_1 {
        for rhs in &numbers_2 {
            let magnitude = (lhs.clone() + rhs.clone()).magnitude();
            if magnitude > max_magnitude {
                max_magnitude = magnitude;
            }
        }
    }

    return max_magnitude;
}

#[derive(PartialEq, Clone)]
pub enum SnailFishNumber {
    NODE { children: Vec<SnailFishNumber> },
    LEAF { val: u8 },
}

impl ToString for SnailFishNumber {
    fn to_string(&self) -> String {
        let mut string = String::new();
        match &self {
            &SnailFishNumber::NODE { children } => {
                string.push('[');
                string.push_str(&children[0].to_string());
                string.push(',');
                string.push_str(&children[1].to_string());
                string.push(']');
            }
            &SnailFishNumber::LEAF { val } => string.push_str(&val.to_string()),
        }
        return string;
    }
}

impl Debug for SnailFishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SnailFishNumber")
            .field("data", &self.to_string())
            .finish()
    }
}

#[derive(Debug)]
enum Explosion {
    NO,
    Left(u8),
    Right(u8),
    Explosion(u8, u8),
    Handled,
}

impl SnailFishNumber {
    fn new_node(left: SnailFishNumber, right: SnailFishNumber) -> SnailFishNumber {
        return SnailFishNumber::NODE {
            children: vec![left, right],
        };
    }

    fn new_value(val: u8) -> SnailFishNumber {
        return SnailFishNumber::LEAF { val };
    }

    fn _is_node(self: &Self) -> bool {
        match self {
            SnailFishNumber::NODE { .. } => true,
            _ => false,
        }
    }

    fn _add_left(self: &mut Self, new_val: u8) {
        match self {
            SnailFishNumber::LEAF { val } => *val += new_val,
            SnailFishNumber::NODE { children } => children[0]._add_left(new_val),
        }
    }

    fn _add_rigth(self: &mut Self, new_val: u8) {
        match self {
            SnailFishNumber::LEAF { val } => *val += new_val,
            SnailFishNumber::NODE { children } => children[1]._add_rigth(new_val),
        }
    }

    fn _explode(self: &mut Self, depth: u8) -> Explosion {
        match self {
            SnailFishNumber::LEAF { .. } => return Explosion::NO,
            SnailFishNumber::NODE { children } => {
                assert_eq!(children.len(), 2);
                if depth >= 4 {
                    // Handle explosion
                    let mut numbers = vec![];
                    for child in children {
                        if let SnailFishNumber::LEAF { val } = child {
                            numbers.push(val);
                        } else {
                            panic!()
                        }
                    }
                    return Explosion::Explosion(*numbers[0], *numbers[1]);
                } else {
                    // Handle recursion
                    // Handle left child
                    let left = children[0]._explode(depth + 1);
                    match left {
                        Explosion::Handled => return Explosion::Handled,
                        Explosion::NO => (),
                        Explosion::Left(val) => return Explosion::Left(val),
                        Explosion::Right(val) => {
                            children[1]._add_left(val);
                            return Explosion::Handled;
                        }
                        Explosion::Explosion(left, right) => {
                            children[0] = SnailFishNumber::LEAF { val: 0 };
                            children[1]._add_left(right);
                            return Explosion::Left(left);
                        }
                    };

                    let right = children[1]._explode(depth + 1);
                    match right {
                        Explosion::Handled => return Explosion::Handled,
                        Explosion::NO => (),
                        Explosion::Right(val) => return Explosion::Right(val),
                        Explosion::Left(val) => {
                            children[0]._add_rigth(val);
                            return Explosion::Handled;
                        }
                        Explosion::Explosion(left, right) => {
                            children[0]._add_rigth(left);
                            children[1] = SnailFishNumber::LEAF { val: 0 };
                            return Explosion::Right(right);
                        }
                    };

                    return Explosion::NO;
                }
            }
        }
    }
}

enum Split {
    Handled,
    No,
    Split(u8),
}

impl SnailFishNumber {
    fn _split(&mut self) -> Split {
        match self {
            Self::NODE { children } => {
                for child in children.iter_mut() {
                    match child._split() {
                        Split::Split(val) => {
                            *child = SnailFishNumber::NODE {
                                children: vec![
                                    SnailFishNumber::LEAF { val: val / 2 },
                                    SnailFishNumber::LEAF {
                                        // Trick to do ceil division
                                        val: (val / 2) + (val % 2 != 0) as u8,
                                    },
                                ],
                            };
                            return Split::Handled;
                        }
                        Split::Handled => return Split::Handled,
                        Split::No => (),
                    };
                }
                return Split::No;
            }
            Self::LEAF { val } => {
                if *val >= 10 {
                    return Split::Split(*val);
                }
                return Split::No;
            }
        }
    }

    pub fn magnitude(&self) -> u32 {
        match self {
            &Self::LEAF { val } => return val as u32,
            Self::NODE { children } => {
                return 3 * children[0].magnitude() + 2 * children[1].magnitude();
            }
        }
    }
}

impl FromStr for SnailFishNumber {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 1 {
            return Ok(SnailFishNumber::new_value(s.parse::<u8>().unwrap()));
        }
        let mut open = 0;
        let mut mid = 0;
        for (i, c) in s.chars().enumerate() {
            match c {
                '[' => open += 1,
                ']' => open -= 1,
                ',' => {
                    if open == 1 {
                        mid = i;
                        break;
                    }
                }
                _ => (),
            };
        }

        let (left, right) = s.split_at(mid);

        let left = left.strip_prefix('[').unwrap();
        let right = right.strip_prefix(',').unwrap().strip_suffix(']').unwrap();

        return Ok(SnailFishNumber::new_node(
            SnailFishNumber::from_str(left).unwrap(),
            SnailFishNumber::from_str(right).unwrap(),
        ));
    }
}

impl Add for SnailFishNumber {
    type Output = Self;
    fn add(self: Self, rhs: Self) -> Self::Output {
        let mut new = SnailFishNumber::new_node(self, rhs);
        while !matches!(new._explode(0), Explosion::NO) || !matches!(new._split(), Split::No) {}
        return new;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "[1,2]",
        SnailFishNumber::new_node(SnailFishNumber::new_value(1), SnailFishNumber::new_value(2))
    )]
    #[case(
        "[3,[4,5]]",
        SnailFishNumber::new_node(
            SnailFishNumber::new_value(3),
            SnailFishNumber::new_node(
                SnailFishNumber::new_value(4),
                SnailFishNumber::new_value(5)
            )
        )
    )]
    #[case(
        "[[4,5],3]",
        SnailFishNumber::new_node(
            SnailFishNumber::new_node(
                SnailFishNumber::new_value(4),
                SnailFishNumber::new_value(5)
            ),
            SnailFishNumber::new_value(3)
        )
    )]
    fn test_snailfishnumber_creation(#[case] string: &str, #[case] expected: SnailFishNumber) {
        assert_eq!(SnailFishNumber::from_str(string).unwrap(), expected)
    }
    #[rstest]
    #[case("[1,2]", "[[3,4],5]", "[[1,2],[[3,4],5]]")]
    #[case("[1,1]", "[2,2]", "[[1,1],[2,2]]")]
    #[case("[[1,1],[2,2]]", "[3,3]", "[[[1,1],[2,2]],[3,3]]")]
    #[case("[[[1,1],[2,2]],[3,3]]", "[4,4]", "[[[[1,1],[2,2]],[3,3]],[4,4]]")]
    #[case(
        "[[[[1,1],[2,2]],[3,3]],[4,4]]",
        "[5,5]",
        "[[[[3,0],[5,3]],[4,4]],[5,5]]"
    )]
    fn test_snailfishnumber_addition(#[case] left: &str, #[case] right: &str, #[case] sum: &str) {
        let left = SnailFishNumber::from_str(left).unwrap();
        let right = SnailFishNumber::from_str(right).unwrap();
        let sum = SnailFishNumber::from_str(sum).unwrap();

        assert_eq!(left + right, sum);
    }

    #[rstest]
    #[case("[1,2]\n[[3,4],5]", "[[1,2],[[3,4],5]]")]
    #[case(
        "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]\n[6,6]",
        "[[[[5,0],[7,4]],[5,5]],[6,6]]"
    )]
    #[case(
        "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]\n[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]\n[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]\n[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]\n[7,[5,[[3,8],[1,4]]]]\n[[2,[2,2]],[8,[8,1]]]\n[2,9]\n[1,[[[9,3],9],[[9,0],[0,7]]]]\n[[[5,[7,4]],7],1]\n[[[[4,2],2],6],[8,7]]",
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
    )]
    #[case(
        "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\n[[[5,[2,8]],4],[5,[[9,9],0]]]\n[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\n[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\n[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\n[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\n[[[[5,4],[7,7]],8],[[8,3],8]]\n[[9,3],[[9,9],[6,[4,9]]]]\n[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\n[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
    )]
    fn test_summation(#[case] input: &str, #[case] expected_sum: &str) {
        assert_eq!(
            parse_input_str(input)
                .into_iter()
                .reduce(|a, b| a + b)
                .unwrap(),
            SnailFishNumber::from_str(expected_sum).unwrap()
        )
    }

    #[rstest]
    #[case("[[1,2],[[3,4],5]]", 143)]
    #[case("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384)]
    #[case("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445)]
    #[case("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791)]
    #[case("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137)]
    #[case("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 3488)]
    fn test_magnitude(#[case] input: &str, #[case] magnitude: u32) {
        let number = SnailFishNumber::from_str(input).unwrap();
        assert_eq!(number.magnitude(), magnitude)
    }

    #[test]
    fn test_part_1() {
        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        assert_eq!(aoc_18_part_1(input), 4140);
    }

    #[test]
    fn test_part_2() {
        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        assert_eq!(aoc_18_part_2(input), 3993);
    }
}
