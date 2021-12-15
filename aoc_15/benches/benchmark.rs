#[cfg(test)]
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

extern crate aoc_15;
use aoc_15::*;

fn speed_test_day_15(c: &mut Criterion) {
    c.bench_function("day_15_part_1", |b| {
        b.iter(|| aoc_15("src/input", black_box(1)))
    });
    c.bench_function("day_15_part_2", |b| {
        b.iter(|| aoc_15("src/input", black_box(5)))
    });
}

fn speed_test_day_15_o_notation(c: &mut Criterion) {
    for i in 1..=10 {
        c.bench_with_input(BenchmarkId::new("day_15", i), &(i), |b, i| {
            b.iter(|| aoc_15("src/input", black_box(*i)))
        });
    }
}

criterion_group!(benches, speed_test_day_15, speed_test_day_15_o_notation);
criterion_main!(benches);
