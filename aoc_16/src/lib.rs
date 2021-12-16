#[derive(PartialEq, Debug)]
enum Message {
    LiretalValue { value: u32 },
    Operator { sub_packets: Vec<Packet> },
}

#[derive(PartialEq, Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    message: Message,
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
        _ => panic!(),
    };
}

fn decode_str(message_str: &str) -> Packet {
    let mut bits = message_str.chars().flat_map(|c| to_bits(c));
    return decode_bits(&mut bits);
}

fn decode_to_value(message_bits: impl Iterator<Item = char>) -> u32 {
    return u32::from_str_radix(&message_bits.collect::<String>(), 2).unwrap();
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
    let value = u32::from_str_radix(&value_str, 2).unwrap();
    return Packet {
        version,
        type_id,
        message: Message::LiretalValue { value },
    };
}

fn decode_sub_packet(mut sub_message_bits: &mut impl Iterator<Item = char>) -> Packet {
    let version = decode_to_value((&mut sub_message_bits).take(3)) as u8;
    let type_id = decode_to_value((&mut sub_message_bits).take(3)) as u8;
    return construct_literal_packet(version, type_id, sub_message_bits);
}

fn decode_bits(message_bits: &mut impl Iterator<Item = char>) -> Packet {
    let version = decode_to_value((message_bits).take(3)) as u8;
    let type_id = decode_to_value((message_bits).take(3)) as u8;

    if type_id == 4 {
        return construct_literal_packet(version, type_id, message_bits);
    } else {
        if message_bits.next().unwrap() == '0' {
            let length = decode_to_value((message_bits).take(15)) as usize;
            let mut sub_message_bits = message_bits.take(length).peekable();
            let mut sub_packets: Vec<Packet> = vec![];
            {
                while sub_message_bits.peek().is_some() {
                    sub_packets.push(decode_sub_packet(&mut sub_message_bits))
                }
            }
            return Packet {
                version,
                type_id,
                message: Message::Operator { sub_packets },
            };
        } else {
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
}
