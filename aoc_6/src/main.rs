use std::collections::HashMap;

fn main() {
    let input_str = String::from("2,5,5,3,2,2,5,1,4,5,2,1,5,5,1,2,3,3,4,1,4,1,4,4,2,1,5,5,3,5,4,3,4,1,5,4,1,5,5,5,4,3,1,2,1,5,1,4,4,1,4,1,3,1,1,1,3,1,1,2,1,3,1,1,1,2,3,5,5,3,2,3,3,2,2,1,3,1,3,1,5,5,1,2,3,2,1,1,2,1,2,1,2,2,1,3,5,4,3,3,2,2,3,1,4,2,2,1,3,4,5,4,2,5,4,1,2,1,3,5,3,3,5,4,1,1,5,2,4,4,1,2,2,5,5,3,1,2,4,3,3,1,4,2,5,1,5,1,2,1,1,1,1,3,5,5,1,5,5,1,2,2,1,2,1,2,1,2,1,4,5,1,2,4,3,3,3,1,5,3,2,2,1,4,2,4,2,3,2,5,1,5,1,1,1,3,1,1,3,5,4,2,5,3,2,2,1,4,5,1,3,2,5,1,2,1,4,1,5,5,1,2,2,1,2,4,5,3,3,1,4,4,3,1,4,2,4,4,3,4,1,4,5,3,1,4,2,2,3,4,4,4,1,4,3,1,3,4,5,1,5,4,4,4,5,5,5,2,1,3,4,3,2,5,3,1,3,2,2,3,1,4,5,3,5,5,3,2,3,1,2,5,2,1,3,1,1,1,5,1");
    let days = 256;
    println!("Part 1: {:?}", aoc_6_part_1(input_str, days));
}

fn aoc_6_part_1(input_str: String, days: u128) -> u128 {
    let mut latern_fish: HashMap<u8, u128> = input_str
        .split(",")
        .map(|s| s.parse::<u8>().unwrap())
        .fold(HashMap::<u8, u128>::new(), |mut m, x| {
            *m.entry(x).or_insert(0) += 1;
            m
        });

    for _i in 1..=days {
        latern_fish = latern_fish
            .iter()
            .fold(HashMap::<u8, u128>::new(), |mut m, (k, v)| {
                match k {
                    0 => {
                        *m.entry(6).or_insert(0) += *v;
                        *m.entry(8).or_insert(0) += *v
                    }
                    _ => *m.entry(*k - 1).or_insert(0) += *v,
                };
                m
            });

    }

    return latern_fish.iter().fold(0, |sum, (_, v)| sum + v);
}

#[test]
fn test_18_days() {
    let input_str = String::from("3,4,3,1,2");
    assert_eq!(aoc_6_part_1(input_str, 18), 26);
}

#[test]
fn test_80_days() {
    let input_str = String::from("3,4,3,1,2");
    assert_eq!(aoc_6_part_1(input_str, 80), 5934);
}

#[test]
fn test_256_days() {
    let input_str = String::from("3,4,3,1,2");
    assert_eq!(aoc_6_part_1(input_str, 256), 26984457539);
}
