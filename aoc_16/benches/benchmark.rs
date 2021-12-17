#[cfg(test)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate aoc_16;
use aoc_16::*;

fn speed_test_day_16(c: &mut Criterion) {
    let message_str = read_file("src/input");
    c.bench_function("day 16 part 1", |b| {
        b.iter(|| aoc_16_part_1(black_box(&message_str)))
    });
    c.bench_function("day 16 part 2", |b| {
        b.iter(|| aoc_16_part_2(black_box(&message_str)))
    });
    c.bench_function("day 16 total", |b| {
        b.iter(|| aoc_16(black_box(&message_str)))
    });

    c.bench_function("day 16 parse packet", |b| {
        b.iter(|| decode_str(black_box(&message_str)))
    });

    let packet = decode_str(&message_str);
    c.bench_function("day 16 count version", |b| {
        b.iter(|| count_versions(black_box(&packet)))
    });
    c.bench_function("day 16 calc value", |b| {
        b.iter(|| calc_value(black_box(&packet)))
    });
}

criterion_group!(benches, speed_test_day_16);
criterion_main!(benches);
