#[cfg(test)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate aoc_13;
use aoc_13::*;

fn speed_test_day_13_with_vis(c: &mut Criterion) {
    let input_str = read_file(String::from("src/input"));
    c.bench_function("day_13_with_vis", |b| b.iter(|| aoc_13(&input_str, true)));
}

fn speed_test_day_13_no_vis(c: &mut Criterion) {
    let input_str = read_file(String::from("src/input"));
    c.bench_function("day_13_no_vis", |b| b.iter(|| aoc_13(&input_str, false)));
}

fn speed_test_parse_data(c: &mut Criterion) {
    let input_str = read_file(String::from("src/input"));
    c.bench_function("parse_data", |b| b.iter(|| parse_inputs(&input_str)));
}
fn speed_test_fold_coordinates(c: &mut Criterion) {
    let input_str = read_file(String::from("src/input"));
    let (coordinates, mut folds) = parse_inputs(&input_str);
    let fold = folds.remove(0);

    c.bench_function("folding", |b| {
        b.iter_batched(
            || coordinates.clone(),
            |coord| fold_coordinates(coord, &fold),
            criterion::BatchSize::PerIteration,
        )
    });
}

criterion_group!(
    benches,
    speed_test_fold_coordinates,
    speed_test_day_13_no_vis,
    speed_test_day_13_with_vis,
    speed_test_parse_data
);
criterion_main!(benches);
