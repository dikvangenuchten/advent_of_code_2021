#![feature(test)]
use std::collections::{HashMap, HashSet};

fn main() {
    println!("Hello, world!");

    let input = String::from(
        "lg-GW
pt-start
pt-uq
nx-lg
ve-GW
start-nx
GW-start
GW-nx
pt-SM
sx-GW
lg-end
nx-SM
lg-SM
pt-nx
end-ve
ve-SM
TG-uq
end-SM
SM-uq",
    );

    let paths_1 = find_paths(&input, recursion_criteria_part_1);
    let paths_2 = find_paths(&input, recursion_criteria_part_2);

    println!("n_paths part 1: {:?}", paths_1.len());
    println!("n_paths part 2: {:?}", paths_2.len());
}

fn find_paths(
    input: &String,
    criterion: fn(&String, &Vec<String>) -> bool,
) -> HashSet<Vec<String>> {
    let connections = create_connections(input);
    println!("connections: {:?}", connections);
    let paths = search_paths_recursive(vec![String::from("start")], &connections, criterion);

    return paths;
}

fn create_connections(input: &String) -> HashMap<String, HashSet<String>> {
    let mut connections = HashMap::<String, HashSet<String>>::new();
    for line in input.lines() {
        let (cave_1, cave_2) = line.split_once("-").unwrap();
        connections
            .entry(String::from(cave_1))
            .or_insert(HashSet::new())
            .insert(String::from(cave_2));
        connections
            .entry(String::from(cave_2))
            .or_insert(HashSet::new())
            .insert(String::from(cave_1));
    }
    return connections;
}

fn is_lowercase(text: &String) -> bool {
    return text.chars().all(|c| char::is_lowercase(c));
}

fn is_uppercase(text: &String) -> bool {
    return text.chars().all(|c| char::is_uppercase(c));
}

fn search_paths_recursive(
    path: Vec<String>,
    connections: &HashMap<String, HashSet<String>>,
    criterion: fn(&String, &Vec<String>) -> bool,
) -> HashSet<Vec<String>> {
    let options = connections.get(path.last().unwrap()).unwrap();

    let mut paths = HashSet::new();
    for next_cave in options {
        let mut current_path = path.clone();
        if next_cave == "end" {
            current_path.push(next_cave.clone());
            paths.insert(current_path);
        } else if next_cave == "start" {
            continue;
        } else if criterion(next_cave, &current_path) {
            current_path.push(next_cave.clone());
            paths.extend(search_paths_recursive(current_path, connections, criterion));
        }
    }

    return paths;
}

fn recursion_criteria_part_1(next_cave: &String, current_path: &Vec<String>) -> bool {
    return is_uppercase(next_cave) || !current_path.contains(next_cave);
}

fn recursion_criteria_part_2(next_cave: &String, current_path: &Vec<String>) -> bool {
    return is_uppercase(next_cave)
        || !current_path.contains(next_cave)
        || (is_lowercase(next_cave)
            && current_path
                .iter()
                .filter(|x| is_lowercase(x))
                .fold(HashMap::<&String, u128>::new(), |mut m, x| {
                    *m.entry(x).or_insert(0) += 1;
                    m
                })
                .iter()
                .all(|(_cave, count)| count < &2));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    extern crate test;
    use test::Bencher;

    #[rstest]
    fn test_n_paths() {
        let input = String::from(
            "start-A
start-b
A-c
A-b
b-d
A-end
b-end
",
        );

        let expected = HashSet::<Vec<String>>::from([
            vec![
                String::from("start"),
                String::from("A"),
                String::from("b"),
                String::from("A"),
                String::from("c"),
                String::from("A"),
                String::from("end"),
            ],
            vec![
                String::from("start"),
                String::from("A"),
                String::from("b"),
                String::from("A"),
                String::from("end"),
            ],
            vec![
                String::from("start"),
                String::from("A"),
                String::from("b"),
                String::from("end"),
            ],
            vec![
                String::from("start"),
                String::from("A"),
                String::from("c"),
                String::from("A"),
                String::from("b"),
                String::from("A"),
                String::from("end"),
            ],
            vec![
                String::from("start"),
                String::from("A"),
                String::from("c"),
                String::from("A"),
                String::from("b"),
                String::from("end"),
            ],
            vec![
                String::from("start"),
                String::from("A"),
                String::from("c"),
                String::from("A"),
                String::from("end"),
            ],
            vec![
                String::from("start"),
                String::from("A"),
                String::from("end"),
            ],
            vec![
                String::from("start"),
                String::from("b"),
                String::from("A"),
                String::from("c"),
                String::from("A"),
                String::from("end"),
            ],
            vec![
                String::from("start"),
                String::from("b"),
                String::from("A"),
                String::from("end"),
            ],
            vec![
                String::from("start"),
                String::from("b"),
                String::from("end"),
            ],
        ]);

        let paths = find_paths(&input, recursion_criteria_part_1);

        assert_eq!(expected, paths)
    }

    #[rstest]
    #[case(
        "dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc",
        19
    )]
    #[case("start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end", 10)]
    #[case(
        "fs-end\nhe-DX\nfs-he\nstart-DX\npj-DX\nend-zg\nzg-sl\nzg-pj\npj-he\nRW-he\nfs-DX
pj-RW\nzg-RW\nstart-pj\nhe-WI\nzg-he\npj-fs\nstart-RW",
        226
    )]
    fn test_path_count(#[case] input: String, #[case] count: usize) {
        assert_eq!(find_paths(&input, recursion_criteria_part_1).len(), count)
    }

    #[rstest]
    #[case("start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end", 36)]
    #[case(
        "dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc",
        103
    )]
    #[case(
        "fs-end\nhe-DX\nfs-he\nstart-DX\npj-DX\nend-zg\nzg-sl\nzg-pj\npj-he\nRW-he\nfs-DX
pj-RW\nzg-RW\nstart-pj\nhe-WI\nzg-he\npj-fs\nstart-RW",
        3509
    )]
    fn test_path_count_v2(#[case] input: String, #[case] count: usize) {
        assert_eq!(find_paths(&input, recursion_criteria_part_2).len(), count)
    }

    #[test]
    fn test_actual() {
        let input = String::from(
            "lg-GW
pt-start
pt-uq
nx-lg
ve-GW
start-nx
GW-start
GW-nx
pt-SM
sx-GW
lg-end
nx-SM
lg-SM
pt-nx
end-ve
ve-SM
TG-uq
end-SM
SM-uq",
        );
        assert_eq!(3708, find_paths(&input, recursion_criteria_part_1).len());
        assert_eq!(93858, find_paths(&input, recursion_criteria_part_2).len());
    }

    #[bench]
    fn bench_day_12_part_1(b: &mut Bencher) {
        let input = String::from(
            "lg-GW
pt-start
pt-uq
nx-lg
ve-GW
start-nx
GW-start
GW-nx
pt-SM
sx-GW
lg-end
nx-SM
lg-SM
pt-nx
end-ve
ve-SM
TG-uq
end-SM
SM-uq"
        );
        b.iter(|| find_paths(&input, recursion_criteria_part_1))
    }

    #[bench]
    fn bench_day_12_part_2(b: &mut Bencher) {
        let input = String::from(
            "lg-GW
pt-start
pt-uq
nx-lg
ve-GW
start-nx
GW-start
GW-nx
pt-SM
sx-GW
lg-end
nx-SM
lg-SM
pt-nx
end-ve
ve-SM
TG-uq
end-SM
SM-uq"
        );
        b.iter(|| find_paths(&input, recursion_criteria_part_2))
    }
}
