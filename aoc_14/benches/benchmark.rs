#[cfg(test)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate aoc_14;
use aoc_14::*;

fn speed_test_day_14(c: &mut Criterion) {
    let input_file = String::from("src/input");
    c.bench_function("day_14_part_1", |b| {
        b.iter(|| aoc_14(black_box(&input_file), black_box(10)))
    });
    c.bench_function("day_14_part_1_2", |b| {
        b.iter(|| aoc_14(black_box(&input_file), black_box(20)))
    });
    c.bench_function("day_14_part_1_3", |b| {
        b.iter(|| aoc_14(black_box(&input_file), black_box(30)))
    });
    c.bench_function("day_14_part_2", |b| {
        b.iter(|| aoc_14(black_box(&input_file), black_box(40)))
    });
}

criterion_group!(benches, speed_test_day_14);
criterion_main!(benches);
