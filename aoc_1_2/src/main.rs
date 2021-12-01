use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("src/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let input: Vec<u16> = contents.lines().map(|s| s.parse::<u16>().unwrap()).collect();
    let mut count: u32 = 0;
    for i in 0..input.len()-3{
        let prev_sum: u16 = input[i..i+3].iter().sum();
        let next_sum: u16 = input[i+1..i+4].iter().sum();
        if prev_sum < next_sum{
            count += 1;
        }
    }
    println!("{:?}", count);

}
