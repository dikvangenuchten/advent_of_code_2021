use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() {
    println!("{:?}", aoc_3_2());
}

fn aoc_3_2() {
    let mut file = File::open("src/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let numbers: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let oxygen = calculate_oxygen(numbers.clone());
    let co2 = calculate_co2(numbers.clone());

    println!("oxygen: {:?}", oxygen);
    println!("co2: {:?}", co2);
    println!("{:?}", oxygen * co2);
}

fn calculate_oxygen(numbers: Vec<Vec<char>>) -> u32 {
    let mut oxy_bits = numbers.clone();
    for i in 0..numbers[0].len() {
        let bit = oxy_bits.iter().map(|x| x[i]).collect::<Vec<char>>();

        let zero_count = bit.iter().filter(|x| **x == '0').count();
        let one_count = bit.len() - zero_count;

        if zero_count > one_count {
            oxy_bits.retain(|x| x[i] == '0');
        } else if one_count == zero_count {
            oxy_bits.retain(|x| x[i] == '1');
        } else {
            oxy_bits.retain(|x| x[i] == '1');
        }
        if oxy_bits.len() == 1 {
            break;
        }
    }
    return u32::from_str_radix(&oxy_bits[0].iter().collect::<String>(), 2).unwrap();
}

fn calculate_co2(numbers: Vec<Vec<char>>) -> u32 {
    let mut co2_bits = numbers.clone();
    for i in 0..numbers[0].len() {
        let bit = co2_bits.iter().map(|x| x[i]).collect::<Vec<char>>();

        let zero_count = bit.iter().filter(|x| **x == '0').count();
        let one_count = bit.len() - zero_count;

        if one_count > zero_count {
            co2_bits.retain(|x| x[i] == '0');
        } else if one_count == zero_count {
            co2_bits.retain(|x| x[i] == '0');
        } else {
            co2_bits.retain(|x| x[i] == '1');
        }
        if co2_bits.len() == 1 {
            break;
        }
    }
    return u32::from_str_radix(&co2_bits[0].iter().collect::<String>(), 2).unwrap();
}
