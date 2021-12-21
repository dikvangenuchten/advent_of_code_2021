use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Read};
use std::str::FromStr;
use std::string::ParseError;

pub fn read_file(file: &str) -> String {
    let input = File::open(file).unwrap();

    let mut contents = String::new();
    BufReader::new(input).read_to_string(&mut contents).unwrap();

    return contents;
}

pub fn aoc_19_comp(input_str: &str) -> (u16, u16) {
    let mut input = parse_file(input_str);
    let mut main_scanner = input.remove(0);
    let mut moved_scanners = vec![main_scanner.clone()];
    while !input.is_empty() {
        let other = input.remove(0);
        if let Ok(other_moved) = main_scanner.join(&other) {
            moved_scanners.push(other_moved);
        } else {
            input.push(other);
        }
    }

    let mut max_distance = 0;
    for scanner1 in &moved_scanners {
        for scanner2 in &moved_scanners {
            let distance = (scanner1.loc[0] - scanner2.loc[0]).abs() as u16
                + (scanner1.loc[1] - scanner2.loc[1]).abs() as u16
                + (scanner1.loc[2] - scanner2.loc[2]).abs() as u16;
            if max_distance < distance {
                max_distance = distance;
            }
        }
    }

    return (main_scanner.beacons.len() as u16, max_distance);
}

#[derive(PartialEq, PartialOrd, Ord, Debug, Eq, Hash, Clone, Copy)]
struct Beacon {
    coords: [i16; 3],
}

impl FromStr for Beacon {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Beacon, Self::Err> {
        let coords: [i16; 3] = s
            .split(",")
            .map(|s| s.parse::<i16>().unwrap())
            .collect::<Vec<i16>>()
            .as_slice()
            .try_into()
            .unwrap();
        return Ok(Beacon { coords });
    }
}

impl Beacon {
    fn apply_rotation(self: &Self, rot_id: u16) -> Beacon {
        let coords = [
            self.coords[0],
            self.coords[1],
            self.coords[2],
            -self.coords[0],
            -self.coords[1],
            -self.coords[2],
        ];
        let permutations = (0..6)
            .permutations(3)
            // Remove any that do not include all axis
            .filter(|coord_idx| {
                HashSet::<usize>::from_iter(coord_idx.iter().map(|x| x % 3)).len() == 3
            })
            .map(|coord_idx| {
                [
                    coords[coord_idx[0]],
                    coords[coord_idx[1]],
                    coords[coord_idx[2]],
                ]
            })
            .collect::<Vec<[i16; 3]>>();

        return Beacon {
            coords: permutations[rot_id as usize],
        };
    }

    fn apply_offset(self: &Self, offset: [i16; 3]) -> Beacon {
        return Beacon {
            coords: [
                self.coords[0] + offset[0],
                self.coords[1] + offset[1],
                self.coords[2] + offset[2],
            ],
        };
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Scanner {
    id: u8,
    beacons: Vec<Beacon>,
    loc: [i16; 3],
}

impl FromStr for Scanner {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Scanner, Self::Err> {
        let mut lines = s.lines();
        // Extract header
        let header = lines.next().unwrap();
        let id = header
            .strip_prefix("--- scanner ")
            .unwrap()
            .strip_suffix(" ---")
            .unwrap()
            .parse::<u8>()
            .unwrap();

        let mut beacons = vec![];
        for beacon_str in lines {
            beacons.push(Beacon::from_str(beacon_str).unwrap())
        }

        return Ok(Scanner {
            id,
            beacons,
            loc: [0, 0, 0],
        });
    }
}

#[derive(Debug)]
struct UnJoinable {
    reason: String,
}

fn calculate_fingerprint(lhs: &Beacon, rhs: &Beacon) -> (u16, i16) {
    return (
        (lhs.coords[0] - rhs.coords[0]).abs() as u16
            + (lhs.coords[1] - rhs.coords[1]).abs() as u16
            + (lhs.coords[2] - rhs.coords[2]).abs() as u16,
        (lhs.coords[0] - rhs.coords[0]).abs().max(
            (lhs.coords[1] - rhs.coords[1])
                .abs()
                .max((lhs.coords[2] - rhs.coords[2]).abs()),
        ),
    );
}

fn find_distances(beacons: &Vec<Beacon>) -> HashMap<&Beacon, HashSet<(u16, i16)>> {
    let mut all_distances = HashMap::new();
    for beacon1 in beacons {
        let mut distances = HashSet::new();
        let mut vec_dist = vec![];
        for beacon2 in beacons {
            distances.insert(calculate_fingerprint(beacon1, beacon2));
            vec_dist.push(calculate_fingerprint(beacon1, beacon2));
        }
        all_distances.insert(beacon1, distances);
    }
    return all_distances;
}

fn find_matching_beacons(
    lhs: &Vec<Beacon>,
    rhs: &Vec<Beacon>,
    threshold: usize,
) -> HashMap<Beacon, Beacon> {
    let lhs_distances = find_distances(&lhs);
    let rhs_distances = find_distances(&rhs);

    let mut matching = HashMap::<Beacon, Beacon>::new();
    for beacon1 in lhs {
        for beacon2 in rhs {
            let overlapping = lhs_distances
                .get(beacon1)
                .unwrap()
                .intersection(rhs_distances.get(beacon2).unwrap())
                .collect::<HashSet<&(u16, i16)>>();
            if overlapping.len() >= threshold {
                matching.insert(beacon1.clone(), beacon2.clone());
                break;
            }
        }
    }
    return matching;
}

fn find_rotation(matching_beacons: &HashMap<Beacon, Beacon>) -> Option<u16> {
    for rotation in 0..48 {
        let mut offset = None;
        let mut correct_flag = true;
        for (beacon_self, beacon_other) in matching_beacons {
            let dist = calculate_fingerprint(beacon_self, &beacon_other.apply_rotation(rotation));
            match offset {
                None => offset = Some(dist),
                Some(offset) => {
                    if offset != dist {
                        correct_flag = false;
                        break;
                    }
                }
            }
        }
        if correct_flag {
            return Some(rotation);
        }
    }
    return None;
}

fn find_offset(matching: &HashMap<Beacon, Beacon>, rotation: u16) -> [i16; 3] {
    let (lhs, rhs) = matching.iter().next().unwrap();
    let rhs = rhs.apply_rotation(rotation);
    return [
        lhs.coords[0] - rhs.coords[0],
        lhs.coords[1] - rhs.coords[1],
        lhs.coords[2] - rhs.coords[2],
    ];
}

impl Scanner {
    fn join(self: &mut Self, other: &Self) -> Result<Self, UnJoinable> {
        let matching_beacons = find_matching_beacons(&self.beacons, &other.beacons, 12);
        if matching_beacons.len() < 11 {
            return Err(UnJoinable {
                reason: String::from("Not enough matches found"),
            });
        }

        let rotation = find_rotation(&matching_beacons);
        if rotation.is_none() {
            return Err(UnJoinable {
                reason: String::from("No rotation found"),
            });
        }
        let rotation = rotation.unwrap();
        let offset = find_offset(&matching_beacons, rotation);

        let other = other.apply_rotation(rotation);
        let other = other.apply_offset(offset);
        let mut other_beacons = other.beacons.clone();
        let mut joined_beacons = self.beacons.clone();
        joined_beacons.append(&mut other_beacons);
        joined_beacons.sort();
        joined_beacons.dedup();

        self.beacons = joined_beacons;

        return Ok(other);
    }

    fn apply_rotation(self: &Self, rotation: u16) -> Scanner {
        return Scanner {
            id: self.id,
            beacons: self
                .beacons
                .iter()
                .map(|b| b.apply_rotation(rotation))
                .collect(),
            loc: [0, 0, 0],
        };
    }

    fn apply_offset(self: &Self, offset: [i16; 3]) -> Scanner {
        return Scanner {
            id: self.id,
            beacons: self
                .beacons
                .iter()
                .map(|b| b.apply_offset(offset))
                .collect(),
            loc: offset,
        };
    }
}

fn parse_file(input_str: &str) -> Vec<Scanner> {
    return input_str
        .split("\n\n")
        .map(|scanner_str| Scanner::from_str(scanner_str).unwrap())
        .collect();
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

    #[rstest]
    #[case("-1,-1,1", Beacon{coords: [-1,-1,1]})]
    #[case("-2,-2,2", Beacon{coords: [-2,-2,2]})]
    #[case("-3,-3,3", Beacon{coords: [-3,-3,3]})]
    #[case("-2,-3,1", Beacon{coords: [-2,-3,1]})]
    #[case("5,6,-4", Beacon{coords: [5,6,-4]})]
    #[case("8,0,7", Beacon{coords:[8,0,7]})]
    fn test_parse_beacon(#[case] input: &str, #[case] expected: Beacon) {
        assert_eq!(Beacon::from_str(input), Ok(expected));
    }

    #[rstest]
    #[case("--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7",Scanner{id: 0, loc:  [0, 0, 0],beacons: vec![
    Beacon{coords: [-1,-1,1]},
    Beacon{coords: [-2,-2,2]},
    Beacon{coords: [-3,-3,3]},
    Beacon{coords: [-2,-3,1]},
    Beacon{coords: [5,6,-4]},
    Beacon{coords:[8,0,7]}]
})]
    fn test_parse_scanner(#[case] input: &str, #[case] expected: Scanner) {
        assert_eq!(Scanner::from_str(input), Ok(expected));
    }

    #[rstest]
    #[case("src/example_input")]
    #[case("src/example_scanner")]
    fn test_parse_file(#[case] input_file: &str) {
        parse_file(&read_file(input_file));
    }

    #[rstest]
    #[case(0, 1)]
    #[case(4, 1)]
    fn test_join(#[case] id_left: usize, #[case] id_right: usize) {
        let scanners = parse_file(&read_file("src/example_input"));
        let mut lhs = scanners[id_left].clone();
        let rhs = scanners[id_right].clone();
        assert!(lhs.join(&rhs).is_ok());
        let lhs = scanners[id_left].clone();
        let mut rhs = scanners[id_right].clone();
        assert!(rhs.join(&lhs).is_ok());
    }

    #[rstest]
    #[case(0, 1, HashMap::from([
        (Beacon {coords: [-618,-824,-621]}, Beacon {coords: [686,422,578]}),
        (Beacon {coords: [-537,-823,-458]}, Beacon {coords: [605,423,415]}),
        (Beacon {coords: [-447,-329,318]}, Beacon {coords: [515,917,-361]}),
        (Beacon {coords: [404,-588,-901]}, Beacon {coords: [-336,658,858]}),
        (Beacon {coords: [544,-627,-890]}, Beacon {coords: [-476,619,847]}),
        (Beacon {coords: [528,-643,409]}, Beacon {coords: [-460,603,-452]}),
        (Beacon {coords: [-661,-816,-575]}, Beacon {coords: [729,430,532]}),
        (Beacon {coords: [390,-675,-793]}, Beacon {coords: [-322,571,750]}),
        (Beacon {coords: [423,-701,434]}, Beacon {coords: [-355,545,-477]}),
        (Beacon {coords: [-345,-311,381]}, Beacon {coords: [413,935,-424]}),
        (Beacon {coords: [459,-707,401]}, Beacon {coords: [-391,539,-444]}),
        (Beacon {coords: [-485,-357,347]}, Beacon {coords: [553,889,-390]}),
            ])
    )]
    fn test_matching(
        #[case] id_left: usize,
        #[case] id_right: usize,
        #[case] mapping: HashMap<Beacon, Beacon>,
    ) {
        let scanners = parse_file(&read_file("src/example_input"));

        // Perspective of lhs
        let matching =
            find_matching_beacons(&scanners[id_left].beacons, &scanners[id_right].beacons, 11);
        assert_eq!(matching.len(), mapping.len());
        assert_eq!(matching, mapping);

        // Perspective of rhs
        let matching =
            find_matching_beacons(&scanners[id_right].beacons, &scanners[id_left].beacons, 11);
        let mapping_reverse = mapping
            .into_iter()
            .map(|(key, value)| (value, key))
            .collect::<HashMap<Beacon, Beacon>>();

        assert_eq!(matching, mapping_reverse);
    }

    #[rstest]
    #[case(HashMap::from([
        (Beacon {coords: [-618,-824,-621]}, Beacon {coords: [686,422,578]}),
        (Beacon {coords: [-537,-823,-458]}, Beacon {coords: [605,423,415]}),
        (Beacon {coords: [-447,-329,318]}, Beacon {coords: [515,917,-361]}),
        (Beacon {coords: [404,-588,-901]}, Beacon {coords: [-336,658,858]}),
        (Beacon {coords: [544,-627,-890]}, Beacon {coords: [-476,619,847]}),
        (Beacon {coords: [528,-643,409]}, Beacon {coords: [-460,603,-452]}),
        (Beacon {coords: [-661,-816,-575]}, Beacon {coords: [729,430,532]}),
        (Beacon {coords: [390,-675,-793]}, Beacon {coords: [-322,571,750]}),
        (Beacon {coords: [423,-701,434]}, Beacon {coords: [-355,545,-477]}),
        (Beacon {coords: [-345,-311,381]}, Beacon {coords: [413,935,-424]}),
        (Beacon {coords: [459,-707,401]}, Beacon {coords: [-391,539,-444]}),
        (Beacon {coords: [-485,-357,347]}, Beacon {coords: [553,889,-390]}),
            ])
    )]
    fn test_find_rotation(#[case] mappings: HashMap<Beacon, Beacon>) {
        assert_eq!(find_rotation(&mappings), Some(25));
    }

    #[rstest]
    #[case("src/example_same_scanner", 0, 0)]
    #[case("src/example_same_scanner", 1, 0)]
    #[case("src/example_same_scanner", 2, 0)]
    #[case("src/example_same_scanner", 3, 0)]
    #[case("src/example_same_scanner", 4, 0)]
    fn test_matching_same(#[case] input_file: &str, #[case] lhs_id: usize, #[case] rhs_id: usize) {
        let input = parse_file(&read_file(input_file));
        let lhs = input[lhs_id].clone();
        let rhs = input[rhs_id].clone();
        assert_eq!(
            find_matching_beacons(&lhs.beacons, &rhs.beacons, 6).len(),
            6
        );
    }

    #[rstest]
    #[case(Beacon {coords: [-618,-824,-621]}, Beacon {coords: [686,422,578]},25, [68, -1246, -43])]
    #[case(Beacon {coords: [-537,-823,-458]}, Beacon {coords: [605,423,415]},25, [68, -1246, -43])]
    #[case(Beacon {coords: [-447,-329,318]}, Beacon {coords: [515,917,-361]},25, [68, -1246, -43])]
    #[case(Beacon {coords: [404,-588,-901]}, Beacon {coords: [-336,658,858]},25, [68, -1246, -43])]
    #[case(Beacon {coords: [544,-627,-890]}, Beacon {coords: [-476,619,847]},25, [68, -1246, -43])]
    #[case(Beacon {coords: [528,-643,409]}, Beacon {coords: [-460,603,-452]},25, [68, -1246, -43])]
    #[case(Beacon {coords: [-661,-816,-575]}, Beacon {coords: [729,430,532]},25, [68, -1246, -43])]
    #[case(Beacon {coords: [390,-675,-793]}, Beacon {coords: [-322,571,750]},25, [68, -1246, -43])]
    #[case(Beacon {coords: [423,-701,434]}, Beacon {coords: [-355,545,-477]},25, [68, -1246, -43])]
    #[case(Beacon {coords: [-345,-311,381]}, Beacon {coords: [413,935,-424]},25, [68, -1246, -43])]
    #[case(Beacon {coords: [459,-707,401]}, Beacon {coords: [-391,539,-444]},25, [68, -1246, -43])]
    #[case(Beacon {coords: [-485,-357,347]}, Beacon {coords: [553,889,-390]},25, [68, -1246, -43])]
    fn test_apply_rotation_and_offset(
        #[case] main: Beacon,
        #[case] secondary: Beacon,
        #[case] rot: u16,
        #[case] offset: [i16; 3],
    ) {
        assert_eq!(main, secondary.apply_rotation(rot).apply_offset(offset));
    }

    #[test]
    fn test_apply_rotation() {
        let mut rotations = vec![];
        for i in 0..48 {
            rotations.push(Beacon { coords: [1, 2, 3] }.apply_rotation(i));
        }

        assert_eq!(
            rotations.len(),
            HashSet::<&Beacon>::from_iter(rotations.iter()).len()
        )
    }

    #[rstest]
    #[case("src/example_input")]
    fn test_part_1_partially(#[case] input_file: &str) {
        let mut input = parse_file(&read_file(input_file));
        let mut main_scanner = input.remove(0);

        println!(
            "Main scanner now has {} beacons",
            main_scanner.beacons.len()
        );
        println!(
            "Joining scanner: {} with {} beacons",
            input[0].id,
            input[0].beacons.len()
        );

        let beacon2 = main_scanner.join(&input[0]);
        assert_eq!(beacon2.unwrap().loc, [68, -1246, -43]);
        let should_be_in = vec![
            Beacon {
                coords: [-618, -824, -621],
            },
            Beacon {
                coords: [-537, -823, -458],
            },
            Beacon {
                coords: [-447, -329, 318],
            },
            Beacon {
                coords: [404, -588, -901],
            },
            Beacon {
                coords: [544, -627, -890],
            },
            Beacon {
                coords: [528, -643, 409],
            },
            Beacon {
                coords: [-661, -816, -575],
            },
            Beacon {
                coords: [390, -675, -793],
            },
            Beacon {
                coords: [423, -701, 434],
            },
            Beacon {
                coords: [-345, -311, 381],
            },
            Beacon {
                coords: [459, -707, 401],
            },
            Beacon {
                coords: [-485, -357, 347],
            },
        ];

        let mut success = true;
        for check in should_be_in {
            if !main_scanner.beacons.contains(&check) {
                println!("{:?} is missing", check);
                success = false;
            };
        }
        if !success {
            println!("Main scanner: {:?}", main_scanner.beacons);
            panic!();
        }

        println!(
            "Main scanner now has {} beacons",
            main_scanner.beacons.len()
        );

        println!("Joining scanner: {}", input[3].id);
        let matching_beacons = find_matching_beacons(&main_scanner.beacons, &input[3].beacons, 10);
        for beacon in matching_beacons {
            println!("{:?}", beacon);
        }

        let beacon2 = main_scanner.join(&input[3]);
        assert_eq!(beacon2.unwrap().loc, [-20, -1133, 1061]);

        println!(
            "Main scanner now has {} beacons",
            main_scanner.beacons.len()
        );

        println!("Joining scanner: {}", input[1].id);
        let beacon2 = main_scanner.join(&input[1]);
        assert_eq!(beacon2.unwrap().loc, [1105, -1205, 1229]);

        println!(
            "Main scanner now has {} beacons",
            main_scanner.beacons.len()
        );

        println!("Joining scanner: {}", input[2].id);
        let beacon2 = main_scanner.join(&input[2]);
        assert_eq!(beacon2.unwrap().loc, [-92, -2380, -20]);
    }

    #[rstest]
    #[case("src/example_input", 79, 3621)]
    #[case("../all_inputs/aoc_19_input.txt", 303, 9621)]
    fn test_day_19(
        #[case] input_file: &str,
        #[case] expected_count: u16,
        #[case] expected_distance: u16,
    ) {
        let input_str = read_file(input_file);
        let (count, max_distance) = aoc_19_comp(&input_str);
        assert_eq!(count, expected_count);
        assert_eq!(max_distance, expected_distance);
    }
}
