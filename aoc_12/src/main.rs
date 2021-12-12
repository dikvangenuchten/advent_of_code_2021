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

    let paths = find_paths(input);

    println!("n_paths: {:?}", paths.len())
}

struct Path {
    path: Vec<String>,
    _double_allowed: bool,
}

impl Path {
    fn double_allowed(self: &Self) -> bool {
        return self._double_allowed;
    }

    fn push(self: &mut Self, cave: String) {
        if is_lowercase(&cave) {
            if self.path.contains(&cave) {
                self._double_allowed = false;
            }
        }
        self.path.push(cave)
    }
}

fn find_paths(input: String) -> HashSet<Vec<String>> {
    let connections = create_connections(input);
    println!("connections: {:?}", connections);
    let paths = search_paths_recursive(vec![String::from("start")], &connections);

    return paths;
}

fn create_connections(input: String) -> HashMap<String, HashSet<String>> {
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
) -> HashSet<Vec<String>> {
    let options = connections.get(path.last().unwrap()).unwrap();

    let mut paths = HashSet::new();
    for next_cave in options {
        let mut current_path = path.clone();
        if next_cave == "end" {
            current_path.push(next_cave.clone());
            paths.insert(current_path);
        } else if is_uppercase(next_cave) {
            current_path.push(next_cave.clone());
            paths.extend(search_paths_recursive(current_path, connections));
        } else if !current_path.contains(next_cave) {
            current_path.push(next_cave.clone());
            paths.extend(search_paths_recursive(current_path, connections));
        }
    }

    return paths;
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

        let paths = find_paths(input);

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
        assert_eq!(find_paths(input).len(), count)
    }

    #[rstest]
    #[case(
        "dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc",
        36
    )]
    #[case("start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end", 103)]
    #[case(
        "fs-end\nhe-DX\nfs-he\nstart-DX\npj-DX\nend-zg\nzg-sl\nzg-pj\npj-he\nRW-he\nfs-DX
pj-RW\nzg-RW\nstart-pj\nhe-WI\nzg-he\npj-fs\nstart-RW",
        3509
    )]
    fn test_path_count_v2(#[case] input: String, #[case] count: usize) {
        assert_eq!(find_paths(input).len(), count)
    }

    #[bench]
    fn bench_day_10(b: &mut Bencher) {
        let input = String::from(
            "start-A
start-b
A-c
A-b
b-d
A-end
b-end",
        );
        // b.iter(|| aoc_12(&input));
    }
}
