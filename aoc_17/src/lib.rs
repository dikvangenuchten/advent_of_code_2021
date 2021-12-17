use itertools::Itertools;
use std::cmp::max;

const GRID: ((i32, i32), (i32, i32)) = ((-100, 500), (-100, 500));

pub fn aoc_17_comp(input: &str) -> (i32, u32) {
    let input = input.trim_end();
    return aoc_17(input, None);
}

pub fn aoc_17(input: &str, grid: Option<((i32, i32), (i32, i32))>) -> (i32, u32) {
    let target = parse_input_str(input);
    let all_locs = grid_search_smart_mt(grid, &target);
    let best_loc = all_locs
        .iter()
        .max_by(|(left_y, _), (right_y, _)| left_y.cmp(right_y))
        .unwrap();
    return (best_loc.0, all_locs.len() as u32);
}

pub fn aoc_17_part_1(input: &str, grid: Option<((i32, i32), (i32, i32))>) -> (i32, (i32, i32)) {
    let target = parse_input_str(input);
    let all_locs = grid_search_smart_mt(grid, &target);
    let best_loc = all_locs
        .iter()
        .max_by(|(left_y, _), (right_y, _)| left_y.cmp(right_y))
        .unwrap();
    return *best_loc;
}

pub fn aoc_17_part_2(input: &str, grid: Option<((i32, i32), (i32, i32))>) -> u32 {
    let target = parse_input_str(input);
    let all_locs = grid_search_smart_mt(grid, &target);
    return all_locs.len() as u32;
}

pub fn parse_input_str(input: &str) -> ((i32, i32), (i32, i32)) {
    let (x_range, y_range) = input
        .trim_start_matches("target area: x=")
        .split_once(", y=")
        .unwrap();
    let (x_left, x_right) = x_range
        .split("..")
        .map(|x| x.parse::<i32>().unwrap())
        .collect_tuple()
        .unwrap();
    let (y_bot, y_top) = y_range
        .split("..")
        .map(|y| y.parse::<i32>().unwrap())
        .collect_tuple()
        .unwrap();
    return ((x_left, x_right), (y_bot, y_top));
}

pub fn grid_search_naive(
    grid: Option<((i32, i32), (i32, i32))>,
    target: &((i32, i32), (i32, i32)),
) -> Vec<(i32, (i32, i32))> {
    let grid = grid.unwrap_or(GRID);

    let mut all_good_vel = vec![];
    for dx in grid.0 .0..grid.0 .1 {
        for dy in grid.1 .0..grid.1 .1 {
            let velocity = (dx, dy);
            if reaches_target(velocity, &target) {
                all_good_vel.push((calc_max_y(velocity), velocity));
            }
        }
    }
    return all_good_vel;
}

pub fn grid_search_smart(
    grid: Option<((i32, i32), (i32, i32))>,
    target: &((i32, i32), (i32, i32)),
) -> Vec<(i32, (i32, i32))> {
    let grid = grid.unwrap_or(GRID);

    let mut possible_dx = vec![];
    let mut possible_dy = vec![];
    for dx in grid.0 .0..grid.0 .1 {
        if reaches_target_x(dx, &target.0) {
            possible_dx.push(dx);
        }
    }
    for dy in grid.1 .0..grid.1 .1 {
        if reaches_target_y(dy, &target.1) {
            possible_dy.push(dy);
        }
    }

    let mut all_good_vel = vec![];
    for dx in possible_dx {
        for dy in &possible_dy {
            let velocity = (dx, *dy);
            if reaches_target(velocity, &target) {
                all_good_vel.push((calc_max_y(velocity), velocity));
            }
        }
    }

    return all_good_vel;
}

pub fn grid_search_smart_mt<'a>(
    grid: Option<((i32, i32), (i32, i32))>,
    target: &'a ((i32, i32), (i32, i32)),
) -> Vec<(i32, (i32, i32))> {
    let grid = grid.unwrap_or(GRID);
    let grid_x = grid.0.clone();
    let target_c = target.clone();
    let dx_thread = std::thread::spawn(move || {
        let target = target_c;
        let mut possible_dx = vec![];
        for dx in grid_x.0..grid_x.1 {
            if reaches_target_x(dx, &target.0) {
                possible_dx.push(dx);
            }
        }
        possible_dx
    });

    let mut possible_dy = vec![];
    for dy in grid.1 .0..grid.1 .1 {
        if reaches_target_y(dy, &target.1) {
            possible_dy.push(dy);
        }
    }

    let possible_dx = dx_thread.join().unwrap();

    let mut all_good_vel = vec![];
    for dx in possible_dx {
        for dy in &possible_dy {
            let velocity = (dx, *dy);
            if reaches_target(velocity, &target) {
                all_good_vel.push((calc_max_y(velocity), velocity));
            }
        }
    }

    return all_good_vel;
}

fn reaches_target_x(mut dx: i32, target: &(i32, i32)) -> bool {
    let mut x = 0;

    loop {
        if target.0 <= x && x <= target.1 {
            return true;
        }
        if target.1 < x || dx == 0 {
            return false;
        }
        x += dx;
        dx -= dx.signum();
    }
}

fn reaches_target_y(mut dy: i32, target: &(i32, i32)) -> bool {
    let mut y = 0;

    loop {
        if target.0 <= y && y <= target.1 {
            return true;
        }
        if y < target.1 {
            return false;
        }
        y += dy;
        dy -= 1;
    }
}

fn reaches_target(mut velocity: (i32, i32), target: &((i32, i32), (i32, i32))) -> bool {
    let mut location = (0, 0);
    loop {
        if is_within_target(&location, &target) {
            return true;
        }
        if is_overshot(&location, &target) {
            return false;
        }
        location.0 += velocity.0;
        location.1 += velocity.1;
        velocity.0 -= velocity.0.signum();
        velocity.1 -= 1;
    }
}

fn calc_max_y(velocity: (i32, i32)) -> i32 {
    return max(0, (velocity.1 * (velocity.1 + 1)) / 2);
}

fn is_within_target(loc: &(i32, i32), target: &((i32, i32), (i32, i32))) -> bool {
    return target.0 .0 <= loc.0
        && loc.0 <= target.0 .1
        && target.1 .0 <= loc.1
        && loc.1 <= target.1 .1;
}

fn is_overshot(loc: &(i32, i32), target: &((i32, i32), (i32, i32))) -> bool {
    return loc.0 > target.0 .1 || loc.1 < target.1 .0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_parse_input_str() {
        let input = "target area: x=240..292, y=-90..-57";
        assert_eq!(((240, 292), (-90, -57)), parse_input_str(input));
    }

    #[rstest]
    #[case((17,-4), ((20, 30), (-10, -5)), false)]
    #[case((23,-10), ((20, 30), (-10, -5)), true)]
    #[case((25,-9), ((20, 30), (-10, -5)), true)]
    #[case((27,-5), ((20, 30), (-10, -5)), true)]
    #[case((29,-6), ((20, 30), (-10, -5)), true)]
    #[case((22,-6), ((20, 30), (-10, -5)), true)]
    #[case((21,-7), ((20, 30), (-10, -5)), true)]
    #[case((9,0   ), ((20, 30), (-10, -5)), true)]
    #[case((27,-7), ((20, 30), (-10, -5)), true)]
    #[case((24,-5), ((20, 30), (-10, -5)), true)]
    #[case((25,-7 ), ((20, 30), (-10, -5)), true)]
    #[case((26,-6), ((20, 30), (-10, -5)), true)]
    #[case((25,-5), ((20, 30), (-10, -5)), true)]
    #[case((6,8  ), ((20, 30), (-10, -5)), true)]
    #[case((11,-2), ((20, 30), (-10, -5)), true)]
    #[case((20,-5), ((20, 30), (-10, -5)), true)]
    #[case((29,-10), ((20, 30), (-10, -5)), true)]
    #[case((6,3  ), ((20, 30), (-10, -5)), true)]
    #[case((28,-7), ((20, 30), (-10, -5)), true)]
    #[case((8,0   ), ((20, 30), (-10, -5)), true)]
    #[case((30,-6), ((20, 30), (-10, -5)), true)]
    #[case((29,-8), ((20, 30), (-10, -5)), true)]
    #[case((20,-10), ((20, 30), (-10, -5)), true)]
    #[case((6,7  ), ((20, 30), (-10, -5)), true)]
    #[case((6,4  ), ((20, 30), (-10, -5)), true)]
    #[case((6,1   ), ((20, 30), (-10, -5)), true)]
    #[case((14,-4), ((20, 30), (-10, -5)), true)]
    #[case((21,-6), ((20, 30), (-10, -5)), true)]
    #[case((26,-10), ((20, 30), (-10, -5)), true)]
    #[case((7,-1 ), ((20, 30), (-10, -5)), true)]
    #[case((7,7  ), ((20, 30), (-10, -5)), true)]
    #[case((8,-1 ), ((20, 30), (-10, -5)), true)]
    #[case((21,-9), ((20, 30), (-10, -5)), true)]
    #[case((6,2  ), ((20, 30), (-10, -5)), true)]
    #[case((20,-7 ), ((20, 30), (-10, -5)), true)]
    #[case((30,-10), ((20, 30), (-10, -5)), true)]
    #[case((14,-3), ((20, 30), (-10, -5)), true)]
    #[case((20,-8 ), ((20, 30), (-10, -5)), true)]
    #[case((13,-2), ((20, 30), (-10, -5)), true)]
    #[case((7,3  ), ((20, 30), (-10, -5)), true)]
    #[case((28,-8), ((20, 30), (-10, -5)), true)]
    #[case((29,-9), ((20, 30), (-10, -5)), true)]
    #[case((15,-3), ((20, 30), (-10, -5)), true)]
    #[case((22,-5 ), ((20, 30), (-10, -5)), true)]
    #[case((26,-8), ((20, 30), (-10, -5)), true)]
    #[case((25,-8), ((20, 30), (-10, -5)), true)]
    #[case((25,-6 ), ((20, 30), (-10, -5)), true)]
    #[case((15,-4), ((20, 30), (-10, -5)), true)]
    #[case((9,-2 ), ((20, 30), (-10, -5)), true)]
    #[case((15,-2), ((20, 30), (-10, -5)), true)]
    #[case((12,-2), ((20, 30), (-10, -5)), true)]
    #[case((28,-9), ((20, 30), (-10, -5)), true)]
    #[case((12,-3 ), ((20, 30), (-10, -5)), true)]
    #[case((24,-6), ((20, 30), (-10, -5)), true)]
    #[case((23,-7), ((20, 30), (-10, -5)), true)]
    #[case((25,-10), ((20, 30), (-10, -5)), true)]
    #[case((7,8  ), ((20, 30), (-10, -5)), true)]
    #[case((11,-3), ((20, 30), (-10, -5)), true)]
    #[case((26,-7), ((20, 30), (-10, -5)), true)]
    #[case((7,1  ), ((20, 30), (-10, -5)), true)]
    #[case((23,-9), ((20, 30), (-10, -5)), true)]
    #[case((6,0   ), ((20, 30), (-10, -5)), true)]
    #[case((22,-10), ((20, 30), (-10, -5)), true)]
    #[case((27,-6), ((20, 30), (-10, -5)), true)]
    #[case((8,1   ), ((20, 30), (-10, -5)), true)]
    #[case((22,-8), ((20, 30), (-10, -5)), true)]
    #[case((13,-4), ((20, 30), (-10, -5)), true)]
    #[case((7,6  ), ((20, 30), (-10, -5)), true)]
    #[case((28,-6), ((20, 30), (-10, -5)), true)]
    #[case((11,-4), ((20, 30), (-10, -5)), true)]
    #[case((12,-4 ), ((20, 30), (-10, -5)), true)]
    #[case((26,-9), ((20, 30), (-10, -5)), true)]
    #[case((7,4), ((20, 30), (-10, -5)), true)]
    #[case((24,-10), ((20, 30), (-10, -5)), true)]
    #[case((23,-8), ((20, 30), (-10, -5)), true)]
    #[case((30,-8), ((20, 30), (-10, -5)), true)]
    #[case((7,0  ), ((20, 30), (-10, -5)), true)]
    #[case((9,-1 ), ((20, 30), (-10, -5)), true)]
    #[case((10,-1), ((20, 30), (-10, -5)), true)]
    #[case((26,-5 ), ((20, 30), (-10, -5)), true)]
    #[case((22,-9), ((20, 30), (-10, -5)), true)]
    #[case((6,5), ((20, 30), (-10, -5)), true)]
    #[case((7,5   ), ((20, 30), (-10, -5)), true)]
    #[case((23,-6), ((20, 30), (-10, -5)), true)]
    #[case((28,-10), ((20, 30), (-10, -5)), true)]
    #[case((10,-2), ((20, 30), (-10, -5)), true)]
    #[case((11,-1), ((20, 30), (-10, -5)), true)]
    #[case((20,-9), ((20, 30), (-10, -5)), true)]
    #[case((14,-2 ), ((20, 30), (-10, -5)), true)]
    #[case((29,-7), ((20, 30), (-10, -5)), true)]
    #[case((13,-3), ((20, 30), (-10, -5)), true)]
    #[case((23,-5 ), ((20, 30), (-10, -5)), true)]
    #[case((24,-8), ((20, 30), (-10, -5)), true)]
    #[case((27,-9), ((20, 30), (-10, -5)), true)]
    #[case((30,-7), ((20, 30), (-10, -5)), true)]
    #[case((28,-5), ((20, 30), (-10, -5)), true)]
    #[case((21,-10), ((20, 30), (-10, -5)), true)]
    #[case((7,9   ), ((20, 30), (-10, -5)), true)]
    #[case((6,6  ), ((20, 30), (-10, -5)), true)]
    #[case((21,-5), ((20, 30), (-10, -5)), true)]
    #[case((27,-10), ((20, 30), (-10, -5)), true)]
    #[case((7,2  ), ((20, 30), (-10, -5)), true)]
    #[case((30,-9), ((20, 30), (-10, -5)), true)]
    #[case((21,-8), ((20, 30), (-10, -5)), true)]
    #[case((22,-7), ((20, 30), (-10, -5)), true)]
    #[case((24,-9), ((20, 30), (-10, -5)), true)]
    #[case((20,-6 ), ((20, 30), (-10, -5)), true)]
    #[case((6,9  ), ((20, 30), (-10, -5)), true)]
    #[case((29,-5), ((20, 30), (-10, -5)), true)]
    #[case((8,-2  ), ((20, 30), (-10, -5)), true)]
    #[case((27,-8), ((20, 30), (-10, -5)), true)]
    #[case((30,-5), ((20, 30), (-10, -5)), true)]
    #[case((24,-7), ((20, 30), (-10, -5)), true)]
    fn test_reach(
        #[case] start_vel: (i32, i32),
        #[case] target: ((i32, i32), (i32, i32)),
        #[case] reaches: bool,
    ) {
        assert_eq!(reaches_target(start_vel, &target), reaches)
    }

    #[rstest]
    fn test_grid_search() {
        let (best_y, _) = aoc_17_part_1("target area: x=20..30, y=-10..-5", None);
        assert_eq!(best_y, 45);
    }

    #[rstest]
    fn test_smart_grid_search() {
        let target = ((20, 30), (-10, -5));
        let grid = GRID;
        let mut all_naive = grid_search_naive(Some(grid), &target);
        let mut all_smart = grid_search_smart(Some(grid), &target);
        all_naive.sort();
        all_smart.sort();
        assert_eq!(all_naive, all_smart)
    }

    #[rstest]
    fn test_smart_grid_search_mt() {
        let target = ((20, 30), (-10, -5));
        let grid = GRID;
        let mut all_smart = grid_search_smart(Some(grid), &target);
        let mut all_smart_mt = grid_search_smart_mt(Some(grid), &target);
        all_smart.sort();
        all_smart_mt.sort();
        assert_eq!(all_smart, all_smart_mt)
    }

    #[rstest]
    fn test_example_input() {
        let input_str = "target area: x=20..30, y=-10..-5";
        let (best_y, num) = aoc_17(input_str, None);
        assert_eq!(best_y, 45);
        assert_eq!(num, 112)
    }

    #[rstest]
    fn test_actual_input() {
        let input_str = "target area: x=240..292, y=-90..-57";
        let (best_y, num) = aoc_17(input_str, None);
        assert_eq!(best_y, 4005);
        assert_eq!(num, 2953)
    }
}
