#![feature(destructuring_assignment)]
use std::fs::File;
use std::io::{BufReader, Read};

use itertools::Itertools;

#[derive(PartialEq, Debug)]
pub enum Message {
    LiretalValue { value: u64 },
    Operator { sub_packets: Vec<Packet> },
}

#[derive(PartialEq, Debug)]
pub struct Packet {
    version: u8,
    type_id: u8,
    message: Message,
}

pub fn read_file(file: &str) -> String {
    let input = File::open(file).unwrap();

    let mut contents = String::new();
    BufReader::new(input).read_to_string(&mut contents).unwrap();

    return contents;
}

pub fn aoc_16_comp(input: &str) -> (u64, u64) {
    return aoc_16(input);
}

pub fn aoc_16(message_str: &str) -> (u64, u64) {
    let packets = decode_str(message_str);
    return (count_versions(&packets), calc_value(&packets));
}

pub fn aoc_16_part_1(message_str: &str) -> u64 {
    let packets = decode_str(message_str);
    return count_versions(&packets);
}

pub fn aoc_16_part_2(message_str: &str) -> u64 {
    let packets = decode_str(message_str);
    return calc_value(&packets);
}

pub fn decode_str(message_str: &str) -> Packet {
    let mut bits = message_str.chars().flat_map(|c| to_bits(c));
    return decode_bits(&mut bits);
}

pub fn count_versions(packet: &Packet) -> u64 {
    let mut sum: u64 = packet.version as u64;
    match &packet.message {
        Message::LiretalValue { value: _ } => (),
        Message::Operator { sub_packets } => {
            sum += sub_packets
                .iter()
                .map(|pack| count_versions(pack))
                .sum::<u64>();
        }
    }
    return sum;
}

pub fn calc_value(packet: &Packet) -> u64 {
    let value = match &packet.message {
        Message::LiretalValue { value } => *value as u64,
        Message::Operator { sub_packets } => match &packet.type_id {
            0 => sub_packets.iter().map(|pack| calc_value(pack)).sum::<u64>(),
            1 => sub_packets
                .iter()
                .map(|pack| calc_value(pack))
                .product::<u64>(),
            2 => sub_packets
                .iter()
                .map(|pack| calc_value(pack))
                .min()
                .unwrap(),
            3 => sub_packets
                .iter()
                .map(|pack| calc_value(pack))
                .max()
                .unwrap(),
            5 => (calc_value(&sub_packets[0]) > calc_value(&sub_packets[1])) as u64,
            6 => (calc_value(&sub_packets[0]) < calc_value(&sub_packets[1])) as u64,
            7 => sub_packets.iter().map(|pack| calc_value(pack)).all_equal() as u64,
            _ => panic!(),
        },
    };
    return value;
}

fn to_bits(letter: char) -> [char; 4] {
    return match letter {
        '0' => ['0', '0', '0', '0'],
        '1' => ['0', '0', '0', '1'],
        '2' => ['0', '0', '1', '0'],
        '3' => ['0', '0', '1', '1'],
        '4' => ['0', '1', '0', '0'],
        '5' => ['0', '1', '0', '1'],
        '6' => ['0', '1', '1', '0'],
        '7' => ['0', '1', '1', '1'],
        '8' => ['1', '0', '0', '0'],
        '9' => ['1', '0', '0', '1'],
        'A' => ['1', '0', '1', '0'],
        'B' => ['1', '0', '1', '1'],
        'C' => ['1', '1', '0', '0'],
        'D' => ['1', '1', '0', '1'],
        'E' => ['1', '1', '1', '0'],
        'F' => ['1', '1', '1', '1'],
        letter => {
            panic!("Unexpected input value: {:?}", letter)
        }
    };
}

fn decode_to_value(message_bits: impl Iterator<Item = char>) -> u64 {
    return u64::from_str_radix(&message_bits.collect::<String>(), 2).unwrap();
}

fn construct_literal_packet(
    version: u8,
    type_id: u8,
    mut message_bits: &mut impl Iterator<Item = char>,
) -> Packet {
    let mut value_str: String = String::from("");
    loop {
        let break_ = (&mut message_bits).next().unwrap().to_digit(2).unwrap();
        value_str.extend(&mut message_bits.take(4));
        if break_ == 0 {
            break;
        };
    }
    let value = u64::from_str_radix(&value_str, 2).unwrap() as u64;
    return Packet {
        version,
        type_id,
        message: Message::LiretalValue { value },
    };
}

fn construct_operator_0(
    version: u8,
    type_id: u8,
    message_bits: &mut impl Iterator<Item = char>,
) -> Packet {
    let length = decode_to_value((message_bits).take(15)) as usize;

    // Need to consume the iterator here otherwise an recursion error occurs.
    // Not completly sure why...
    let sub_message = message_bits.take(length).collect::<String>();
    let mut sub_message_bits = sub_message.chars().peekable();
    // let mut sub_message_bits = message_bits.take(length).peekable();
    let mut sub_packets: Vec<Packet> = vec![];

    while sub_message_bits.peek().is_some() {
        let sub_version = decode_to_value((&mut sub_message_bits).take(3)) as u8;
        let sub_type_id = decode_to_value((&mut sub_message_bits).take(3)) as u8;

        if sub_type_id == 4 {
            sub_packets.push(construct_literal_packet(
                sub_version,
                sub_type_id,
                &mut sub_message_bits,
            ));
        } else {
            if (&mut sub_message_bits).next().unwrap() == '0' {
                sub_packets.push(construct_operator_0(
                    sub_version,
                    sub_type_id,
                    &mut sub_message_bits,
                ));
            } else {
                sub_packets.push(construct_operator_1(
                    sub_version,
                    sub_type_id,
                    &mut sub_message_bits,
                ));
            }
        }
    }

    return Packet {
        version,
        type_id,
        message: Message::Operator { sub_packets },
    };
}

fn construct_operator_1(
    version: u8,
    type_id: u8,
    message_bits: &mut impl Iterator<Item = char>,
) -> Packet {
    let num_sub = decode_to_value((message_bits).take(11)) as u16;

    let mut sub_packets = vec![];
    for _ in 0..num_sub {
        sub_packets.push(decode_bits(message_bits));
    }
    return Packet {
        version,
        type_id,
        message: Message::Operator { sub_packets },
    };
}

fn decode_bits(message_bits: &mut impl Iterator<Item = char>) -> Packet {
    let version = decode_to_value((message_bits).take(3)) as u8;
    let type_id = decode_to_value((message_bits).take(3)) as u8;

    if type_id == 4 {
        return construct_literal_packet(version, type_id, message_bits);
    } else {
        if message_bits.next().unwrap() == '0' {
            return construct_operator_0(version, type_id, message_bits);
        } else {
            return construct_operator_1(version, type_id, message_bits);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("D2FE28", Packet{version: 6, type_id:4, message: Message::LiretalValue{value: 2021}})]
    #[case("38006F45291200", Packet{version: 1, type_id:6, message: Message::Operator{sub_packets: 
        vec![
            Packet{version: 6, type_id:4, message: Message::LiretalValue{value: 10}},
            Packet{version: 2, type_id:4, message: Message::LiretalValue{value: 20}},
        ]
    }})]
    #[case("EE00D40C823060", Packet{version: 7, type_id:3, message: Message::Operator{ sub_packets:
        vec![
            Packet{version: 2, type_id:4, message: Message::LiretalValue{value: 1}},
            Packet{version: 4, type_id:4, message: Message::LiretalValue{value: 2}},
            Packet{version: 1, type_id:4, message: Message::LiretalValue{value: 3}}
        ]
    }})]
    fn test_example_decoding(#[case] encoded: &str, #[case] expected_packet: Packet) {
        assert_eq!(expected_packet, decode_str(encoded))
    }

    #[rstest]
    #[case("8A004A801A8002F478", 16)]
    #[case("620080001611562C8802118E34", 12)]
    #[case("C0015000016115A2E0802F182340", 23)]
    #[case("A0016C880162017C3686B18A3D4780", 31)]
    fn test_example_counts(#[case] encoded: &str, #[case] version_sum: u64) {
        assert_eq!(version_sum, aoc_16_part_1(encoded))
    }

    #[rstest]
    #[case("C200B40A82", 3)]
    #[case("04005AC33890", 54)]
    #[case("880086C3E88112", 7)]
    #[case("CE00C43D881120", 9)]
    #[case("D8005AC2A8F0", 1)]
    #[case("F600BC2D8F", 0)]
    #[case("9C005AC2F8F0", 0)]
    #[case("9C0141080250320F1802104A08", 1)]
    fn test_example_calcs(#[case] encoded: &str, #[case] version_sum: u64) {
        assert_eq!(version_sum, aoc_16_part_2(encoded))
    }

    #[rstest]
    fn test_actual_input() {
        let input = read_file("src/input");
        let (part_1, part_2) = aoc_16(&input);
        assert_eq!(860, part_1);
        assert_eq!(470949537659, part_2);
    }
}
