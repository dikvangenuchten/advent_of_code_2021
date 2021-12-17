#[cfg(test)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate aoc_17;
use aoc_17::*;

fn speed_test_day_17(c: &mut Criterion) {
    let input = "target area: x=240..292, y=-90..-57";
    c.bench_function("day 17 part 1", |b| {
        b.iter(|| aoc_17_part_1(black_box(&input), None))
    });
    c.bench_function("day 17 part 2", |b| {
        b.iter(|| aoc_17_part_2(black_box(&input), None))
    });
    c.bench_function("day 17 total", |b| {
        b.iter(|| aoc_17(black_box(&input), None))
    });
}

fn speed_test_grid_search(c: &mut Criterion) {
    let input = "target area: x=240..292, y=-90..-57";
    let mut group = c.benchmark_group("Day 17 grid search");
    let target = parse_input_str(input);
    group.bench_function("day 17 grid search naive", |b| {
        b.iter(|| grid_search_naive(None, &target))
    });
    group.bench_function("day 17 grid search smart", |b| {
        b.iter(|| grid_search_smart(None, &target))
    });
    group.bench_function("day 17 grid search smart mt", |b| {
        b.iter(|| grid_search_smart_mt(None, &target))
    });
}

criterion_group!(benches, speed_test_day_17, speed_test_grid_search);
criterion_main!(benches);
