use aoc2025::days::*;
use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day01a", |b| b.iter(|| day01a::get_result()));
    c.bench_function("day01b", |b| b.iter(|| day01b::get_result()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
