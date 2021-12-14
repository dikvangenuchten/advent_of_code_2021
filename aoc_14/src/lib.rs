use itertools::Itertools;
use itertools::MinMaxResult;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};

pub fn aoc_14(file_input: &str, iterations: u8) -> u64 {
    let input = read_file(file_input);

    let (template_str, conversions) = parse_input_v2(&input);

    let mut whole_polymer = parse_template_v2(&template_str);

    for _ in 0..iterations {
        whole_polymer = react_v2(whole_polymer, &conversions);
    }

    return diff(
        frequency_v2(&whole_polymer)
            .iter()
            .minmax_by(|(_, count_1), (_, count_2)| count_1.cmp(count_2)),
    );
}

pub fn read_file(file: &str) -> String {
    let input = File::open(file).unwrap();

    let mut contents = String::new();
    BufReader::new(input).read_to_string(&mut contents).unwrap();

    return contents;
}

pub fn parse_input(input_str: &str) -> (&str, HashMap<(char, char), char>) {
    let (template_str, conversions_str) = input_str.split_once("\n\n").unwrap();
    let conversion = parse_conversion(conversions_str);
    return (template_str, conversion);
}

pub fn parse_input_v2(input_str: &str) -> (String, HashMap<(char, char), char>) {
    let (template_str, conversions_str) = input_str.split_once("\n\n").unwrap();
    let conversion = parse_conversion(conversions_str);

    let mut template = template_str.to_string();
    template.push(' ');
    return (template, conversion);
}

pub fn diff(result: MinMaxResult<(&char, &u64)>) -> u64 {
    match result {
        MinMaxResult::MinMax(min, max) => max.1 - min.1,
        _ => 0,
    }
}

fn parse_conversion(conversion_str: &str) -> HashMap<(char, char), char> {
    return conversion_str
        .lines()
        .map(|s| s.split_once(" -> ").unwrap())
        .map(|(pol, c)| {
            let mut pol_iter = pol.chars();
            (
                (pol_iter.next().unwrap(), pol_iter.next().unwrap()),
                c.chars().next().unwrap(),
            )
        })
        .collect::<HashMap<(char, char), char>>();
}

pub fn react<'a>(
    template: impl Iterator<Item = char> + 'a + Sized,
    conversion: &'a HashMap<(char, char), char>,
) -> impl Iterator<Item = char> + 'a + Sized {
    let (t1, t2) = template.tee();
    return t1.interleave(
        t2.tuple_windows()
            .map(|polymer: (char, char)| *conversion.get(&polymer).unwrap()),
    );
}

pub fn parse_template_v2(template: &str) -> HashMap<(char, char), u64> {
    template
        .chars()
        .tuple_windows()
        .fold(HashMap::new(), |mut m, polymer| {
            *m.entry(polymer).or_insert(0) += 1;
            m
        })
}

pub fn react_v2<'a>(
    template: HashMap<(char, char), u64>,
    conversion: &'a HashMap<(char, char), char>,
) -> HashMap<(char, char), u64> {
    return template
        .iter()
        .fold(HashMap::new(), |mut m, (polymer, count)| {
            let new_polymer = conversion.get(polymer);
            match new_polymer {
                Some(new_polymer) => {
                    *m.entry((polymer.0, *new_polymer)).or_insert(0) += count;
                    *m.entry((*new_polymer, polymer.1)).or_insert(0) += count;
                }
                None => {
                    *m.entry(*polymer).or_insert(0) += count;
                }
            }
            m
        });
}

pub fn frequency(template: impl Iterator<Item = char>) -> HashMap<char, u64> {
    template.fold(HashMap::new(), |mut m, c| {
        *m.entry(c).or_insert(0) += 1;
        m
    })
}

pub fn frequency_v2(template: &HashMap<(char, char), u64>) -> HashMap<char, u64> {
    return template
        .iter()
        .fold(HashMap::new(), |mut m, (polymer, count)| {
            *m.entry(polymer.0).or_insert(0) += count;
            m
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example_parse_input() {
        let input = "NNCB\n\nCH -> B\nHH -> N\nCB -> H\nNH -> C\nHB -> C\nHC -> B\nHN -> C\nNN -> C\nBH -> H\nNC -> B\nNB -> B\nBN -> B\nBB -> N\nBC -> B\nCC -> N\nCN -> C";

        let (template, conversions) = parse_input(input);
        assert_eq!(template, "NNCB");

        assert_eq!(
            conversions,
            HashMap::from([
                (('C', 'H'), 'B'),
                (('H', 'H'), 'N'),
                (('C', 'B'), 'H'),
                (('N', 'H'), 'C'),
                (('H', 'B'), 'C'),
                (('H', 'C'), 'B'),
                (('H', 'N'), 'C'),
                (('N', 'N'), 'C'),
                (('B', 'H'), 'H'),
                (('N', 'C'), 'B'),
                (('N', 'B'), 'B'),
                (('B', 'N'), 'B'),
                (('B', 'B'), 'N'),
                (('B', 'C'), 'B'),
                (('C', 'C'), 'N'),
                (('C', 'N'), 'C'),
            ])
        )
    }

    #[test]
    fn test_example_iteration() {
        let input = "NNCB\n\nCH -> B\nHH -> N\nCB -> H\nNH -> C\nHB -> C\nHC -> B\nHN -> C\nNN -> C\nBH -> H\nNC -> B\nNB -> B\nBN -> B\nBB -> N\nBC -> B\nCC -> N\nCN -> C";

        let (template, conversions) = parse_input(input);

        let (collect, after_1_iteration) = react(template.chars(), &conversions).tee();
        assert_eq!(
            collect.collect::<Vec<_>>(),
            "NCNBCHB".chars().collect::<Vec<_>>()
        );

        let (collect, after_2_iteration) = react(after_1_iteration, &conversions).tee();
        assert_eq!(
            collect.collect::<Vec<_>>(),
            "NBCCNBBBCBHCB".chars().collect::<Vec<_>>()
        );

        let (collect, after_3_iteration) = react(after_2_iteration, &conversions).tee();
        assert_eq!(
            collect.collect::<Vec<_>>(),
            "NBBBCNCCNBBNBNBBCHBHHBCHB".chars().collect::<Vec<_>>()
        );

        let collect = react(after_3_iteration, &conversions);
        assert_eq!(
            collect.collect::<Vec<_>>(),
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
                .chars()
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_frequency() {
        assert_eq!(
            frequency("NCNBCHB".chars()),
            HashMap::from([('N', 2), ('C', 2), ('B', 2), ('H', 1)])
        )
    }

    #[test]
    fn test_frequency_v2() {
        let input = parse_template_v2("NCNBCHB ");
        assert_eq!(frequency("NCNBCHB".chars()), frequency_v2(&input))
    }

    #[test]
    fn test_react_v2() {
        let input = "NNCB \n\nCH -> B\nHH -> N\nCB -> H\nNH -> C\nHB -> C\nHC -> B\nHN -> C\nNN -> C\nBH -> H\nNC -> B\nNB -> B\nBN -> B\nBB -> N\nBC -> B\nCC -> N\nCN -> C";

        let (template, conversions) = parse_input(input);

        let after_iteration = parse_template_v2(template);
        assert_eq!(frequency_v2(&after_iteration), frequency("NNCB".chars()));

        let after_iteration = react_v2(after_iteration, &conversions);
        assert_eq!(frequency_v2(&after_iteration), frequency("NCNBCHB".chars()));

        let after_iteration = react_v2(after_iteration, &conversions);
        assert_eq!(
            frequency_v2(&after_iteration),
            frequency("NBCCNBBBCBHCB".chars())
        );

        let after_iteration = react_v2(after_iteration, &conversions);
        assert_eq!(
            frequency_v2(&after_iteration),
            frequency("NBBBCNCCNBBNBNBBCHBHHBCHB".chars())
        );

        let after_iteration = react_v2(after_iteration, &conversions);
        assert_eq!(
            frequency_v2(&after_iteration),
            frequency("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB".chars())
        );
    }
}
