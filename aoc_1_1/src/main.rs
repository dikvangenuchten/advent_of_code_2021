use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("src/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut prev_value: Option<u16> = None;
    let mut count: u32 = 0;
    for line in contents.lines() {
        let value = line.parse::<u16>().unwrap();
        println!("{}", value);
        match prev_value {
            Some(prev_value) => count += if prev_value < value {1} else {0},
            None => ()
        }
        prev_value = Some(value);
    }
    println!("{}", count)
}
