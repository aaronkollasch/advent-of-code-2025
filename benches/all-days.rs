use aoc2025::days::*;
use criterion::{Criterion, criterion_group, criterion_main};

#[rustfmt::skip]
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day01a", |b| b.iter(|| day01a::get_result(include_bytes!("../inputs/day01.txt"))));
    c.bench_function("day01b", |b| b.iter(|| day01b::get_result(include_bytes!("../inputs/day01.txt"))));
    c.bench_function("day02a", |b| b.iter(|| day02a::get_result(include_bytes!("../inputs/day02.txt"))));
    c.bench_function("day02b", |b| b.iter(|| day02b::get_result(include_bytes!("../inputs/day02.txt"))));
    c.bench_function("day03a", |b| b.iter(|| day03a::get_result(include_bytes!("../inputs/day03.txt"))));
    c.bench_function("day03b", |b| b.iter(|| day03b::get_result(include_bytes!("../inputs/day03.txt"))));
    c.bench_function("day04a", |b| b.iter(|| day04a::get_result(include_bytes!("../inputs/day04.txt"))));
    c.bench_function("day04b", |b| b.iter(|| day04b::get_result(include_bytes!("../inputs/day04.txt"))));
    c.bench_function("day05a", |b| b.iter(|| day05a::get_result(include_bytes!("../inputs/day05.txt"))));
    c.bench_function("day05b", |b| b.iter(|| day05b::get_result(include_bytes!("../inputs/day05.txt"))));
    c.bench_function("day06a", |b| b.iter(|| day06a::get_result(include_bytes!("../inputs/day06.txt"))));
    c.bench_function("day06b", |b| b.iter(|| day06b::get_result(include_bytes!("../inputs/day06.txt"))));
    c.bench_function("day07a", |b| b.iter(|| day07a::get_result(include_bytes!("../inputs/day07.txt"))));
    c.bench_function("day07b", |b| b.iter(|| day07b::get_result(include_bytes!("../inputs/day07.txt"))));
    // c.bench_function("day08a", |b| b.iter(|| day08a::get_result(include_bytes!("../inputs/day08.txt"))));
    // c.bench_function("day08b", |b| b.iter(|| day08b::get_result(include_bytes!("../inputs/day08.txt"))));
    // c.bench_function("day09a", |b| b.iter(|| day09a::get_result(include_bytes!("../inputs/day09.txt"))));
    // c.bench_function("day09b", |b| b.iter(|| day09b::get_result(include_bytes!("../inputs/day09.txt"))));
    // c.bench_function("day10a", |b| b.iter(|| day10a::get_result(include_bytes!("../inputs/day10.txt"))));
    // c.bench_function("day10b", |b| b.iter(|| day10b::get_result(include_bytes!("../inputs/day10.txt"))));
    // c.bench_function("day11a", |b| b.iter(|| day11a::get_result(include_bytes!("../inputs/day11.txt"))));
    // c.bench_function("day11b", |b| b.iter(|| day11b::get_result(include_bytes!("../inputs/day11.txt"))));
    // c.bench_function("day12a", |b| b.iter(|| day12a::get_result(include_bytes!("../inputs/day12.txt"))));
    // c.bench_function("day12b", |b| b.iter(|| day12b::get_result(include_bytes!("../inputs/day12.txt"))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
