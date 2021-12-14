#[cfg(test)]
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

extern crate aoc_14;
use aoc_14::*;

fn speed_test_day_14(c: &mut Criterion) {
    let input_file = String::from("src/input");
    c.bench_function("day_14_part_1", |b| {
        b.iter(|| aoc_14(black_box(&input_file), black_box(10)))
    });
    c.bench_function("day_14_part_2", |b| {
        b.iter(|| aoc_14(black_box(&input_file), black_box(40)))
    });
}

fn speed_test_day_14_o_notation(c: &mut Criterion) {
    let input_file = String::from("src/input");

    for i in 1..=10 {
        c.bench_with_input(BenchmarkId::new("day_14", i), &(10 * i), |b, i| {
            b.iter(|| aoc_14(black_box(&input_file), black_box(*i)))
        });
    }
}

criterion_group!(benches, speed_test_day_14, speed_test_day_14_o_notation);
criterion_main!(benches);
