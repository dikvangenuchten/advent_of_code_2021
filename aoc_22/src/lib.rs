use std::str::FromStr;
use std::string::ParseError;

use std::fs::File;
use std::io::{BufReader, Read};
pub fn read_file(file: &str) -> String {
    let input = File::open(file).unwrap();

    let mut contents = String::new();
    BufReader::new(input).read_to_string(&mut contents).unwrap();

    return contents;
}

pub fn aoc_22_comp(input_str: &str) -> (u64, u64) {
    let reactor = Reactor::from_str(&input_str).unwrap();
    let region = ((-50, 50), (-50, 50), (-50, 50));
    let part_1 = reactor.count_region(region);

    let region = reactor.commands.iter().map(|c| (c.x, c.y, c.z)).fold(
        ((-50, 50), (-50, 50), (-50, 50)),
        |current, c| {
            (
                (current.0 .0.min(c.0 .0), current.0 .1.max(c.0 .1)),
                (current.1 .0.min(c.1 .0), current.1 .1.max(c.1 .1)),
                (current.2 .0.min(c.2 .0), current.2 .1.max(c.2 .1)),
            )
        },
    );
    let part_2 = reactor.count_region(region);
    return (part_1, part_2);
}

struct Reactor {
    commands: Vec<ReactorCommand>,
}

impl Reactor {
    fn count_region(&self, region: ((i32, i32), (i32, i32), (i32, i32))) -> u64 {
        let mut count = 0;
        for i in region.0 .0..=region.0 .1 {
            for j in region.1 .0..=region.1 .1 {
                for k in region.2 .0..=region.2 .1 {
                    count += self.check_loc((i, j, k)) as u64;
                }
            }
        }
        return count;
    }

    fn check_loc(&self, loc: (i32, i32, i32)) -> u8 {
        let mut on = false;
        for command in &self.commands {
            if command.contains(loc) {
                on = command.on;
            }
        }
        return on as u8;
    }
}

impl FromStr for Reactor {
    type Err = ParseError;
    fn from_str(s: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let mut commands = vec![];
        for command_str in s.lines() {
            commands.push(ReactorCommand::from_str(command_str)?)
        }

        return Ok(Reactor { commands });
    }
}

#[derive(Debug, PartialEq)]
struct ReactorCommand {
    on: bool,
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

impl ReactorCommand {
    fn contains(&self, index: (i32, i32, i32)) -> bool {
        return self.x.0 <= index.0
            && index.0 <= self.x.1
            && self.y.0 <= index.1
            && index.1 <= self.y.1
            && self.z.0 <= index.2
            && index.2 <= self.z.1;
    }
}

impl FromStr for ReactorCommand {
    type Err = ParseError;

    fn from_str(s: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let mut s = s;
        let on;
        if s.starts_with("on") {
            s = s.strip_prefix("on ").unwrap();
            on = true;
        } else {
            s = s.strip_prefix("off ").unwrap();
            on = false;
        }

        let (x_min, s) = s.strip_prefix("x=").unwrap().split_once("..").unwrap();
        let (x_max, s) = s.split_once(",").unwrap();
        let (y_min, s) = s.strip_prefix("y=").unwrap().split_once("..").unwrap();
        let (y_max, s) = s.split_once(",").unwrap();
        let (z_min, z_max) = s.strip_prefix("z=").unwrap().split_once("..").unwrap();

        return Ok(ReactorCommand {
            on: on,
            x: (x_min.parse().unwrap(), x_max.parse().unwrap()),
            y: (y_min.parse().unwrap(), y_max.parse().unwrap()),
            z: (z_min.parse().unwrap(), z_max.parse().unwrap()),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("on x=-20..26,y=-36..17,z=-47..7",       ReactorCommand {on: true,x:(-20,26, ),  y:(-36,17),    z:(-47,7)})]
    #[case("on x=-20..33,y=-21..23,z=-26..28",      ReactorCommand {on: true,x:(-20,33, ),  y:(-21,23, ),   z:(-26,28)})]
    #[case("on x=-22..28,y=-29..23,z=-38..16",      ReactorCommand {on: true,x:(-22,28, ),  y:(-29,23, ),   z:(-38,16)})]
    #[case("on x=-46..7,y=-6..46,z=-50..-1",        ReactorCommand {on: true,x:(-46,7,  ),  y:(-6,46,  ),   z:(-50,-1)})]
    #[case("on x=-49..1,y=-3..46,z=-24..28",        ReactorCommand {on: true,x:(-49,1,  ),  y:(-3,46,  ),   z:(-24,28)})]
    #[case("on x=2..47,y=-22..22,z=-23..27",        ReactorCommand {on: true,x:(2,47,   ),  y:(-22,22, ),   z:(-23,27)})]
    #[case("on x=-27..23,y=-28..26,z=-21..29",      ReactorCommand {on: true,x:(-27,23, ),  y:(-28,26, ),   z:(-21,29)})]
    #[case("on x=-39..5,y=-6..47,z=-3..44",         ReactorCommand {on: true,x:(-39,5,  ),  y:(-6,47,  ),   z:(-3,44)})]
    #[case("on x=-30..21,y=-8..43,z=-13..34",       ReactorCommand {on: true,x:(-30,21, ),  y:(-8,43,  ),   z:(-13,34)})]
    #[case("on x=-22..26,y=-27..20,z=-29..19",      ReactorCommand {on: true,x:(-22,26, ),  y:(-27,20, ),   z:(-29,19)})]
    #[case("off x=-48..-32,y=26..41,z=-47..-37",    ReactorCommand {on: false,x:(-48,-32,),  y:(26,41,  ),   z:(-47,-37)})]
    #[case("on x=-12..35,y=6..50,z=-50..-2",        ReactorCommand {on: true,x:(-12,35, ),  y:(6,50,   ),   z:(-50,-2)})]
    #[case("off x=-48..-32,y=-32..-16,z=-15..-5",   ReactorCommand {on: false,x:(-48,-32,),  y:(-32,-16,),   z:(-15,-5)})]
    #[case("on x=-18..26,y=-33..15,z=-7..46",       ReactorCommand {on: true,x:(-18,26, ),  y:(-33,15, ),   z:(-7,46)})]
    #[case("off x=-40..-22,y=-38..-28,z=23..41",    ReactorCommand {on: false,x:(-40,-22,),  y:(-38,-28,),   z:(23,41)})]
    #[case("on x=-16..35,y=-41..10,z=-47..6",       ReactorCommand {on: true,x:(-16,35, ),  y:(-41,10, ),   z:(-47,6)})]
    #[case("off x=-32..-23,y=11..30,z=-14..3",      ReactorCommand {on: false,x:(-32,-23,),  y:(11,30,  ),   z:(-14,3)})]
    #[case("on x=-49..-5,y=-3..45,z=-29..18",       ReactorCommand {on: true,x:(-49,-5, ),  y:(-3,45,  ),   z:(-29,18)})]
    #[case("off x=18..30,y=-20..-8,z=-3..13",       ReactorCommand {on: false,x:(18,30,  ),  y:(-20,-8, ),   z:(-3,13)})]
    #[case("on x=-41..9,y=-7..43,z=-33..15",        ReactorCommand {on: true,x:(-41,9,  ),  y:(-7,43,  ),   z:(-33,15)})]
    fn test_parse_command(#[case] command_str: &str, #[case] command: ReactorCommand) {
        assert_eq!(ReactorCommand::from_str(command_str), Ok(command));
    }

    #[rstest]
    #[case("src/example_input", 474140, Some(((-50, 50), (-50, 50), (-50, 50))))]
    #[case("src/input", 642125, Some(((-50, 50), (-50, 50), (-50, 50))))]
    #[case("src/example_input", 2758514936282235, None)]
    #[case("src/input", 642125, None)]
    fn test_reactor(
        #[case] input_file: &str,
        #[case] expected_count: u64,
        #[case] mut region: Option<((i32, i32), (i32, i32), (i32, i32))>,
    ) {
        let input_str = read_file(input_file);
        let reactor = Reactor::from_str(&input_str).unwrap();
        if region.is_none() {
            region = Some(reactor.commands.iter().map(|c| (c.x, c.y, c.z)).fold(
                ((-50, 50), (-50, 50), (-50, 50)),
                |current, c| {
                    (
                        (current.0 .0.min(c.0 .0), current.0 .1.max(c.0 .1)),
                        (current.1 .0.min(c.1 .0), current.1 .1.max(c.1 .1)),
                        (current.2 .0.min(c.2 .0), current.2 .1.max(c.2 .1)),
                    )
                },
            ));
        }

        assert_eq!(reactor.count_region(region.unwrap()), expected_count)
    }
}
