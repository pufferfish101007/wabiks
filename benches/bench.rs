use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wabiks::{Rubiks, Side, Direction};
use Direction::*;
use Side::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("benches");
    let r  = Rubiks::solved();
    group.bench_function("move up anticlockwise", |b| b.iter(|| r.move_side(black_box(Side::Up, Direction::Anticlockwise))));
    group.bench_function("move up twice", |b| b.iter(|| r.move_side(black_box(Side::Up, Direction::Twice))));
    group.bench_function("move up clockwise", |b| b.iter(|| r.move_side(black_box(Side::Up, Direction::Clockwise))));
    group.bench_function("move down anticlockwise", |b| b.iter(|| r.move_side(black_box(Side::Down, Direction::Anticlockwise))));
    group.bench_function("move down twice", |b| b.iter(|| r.move_side(black_box(Side::Down, Direction::Twice))));
    group.bench_function("move down clockwise", |b| b.iter(|| r.move_side(black_box(Side::Down, Direction::Clockwise))));
    group.bench_function("move left anticlockwise", |b| b.iter(|| r.move_side(black_box(Side::Left, Direction::Anticlockwise))));
    group.bench_function("move left twice", |b| b.iter(|| r.move_side(black_box(Side::Left, Direction::Twice))));
    group.bench_function("move left clockwise", |b| b.iter(|| r.move_side(black_box(Side::Left, Direction::Clockwise))));
    group.bench_function("move right anticlockwise", |b| b.iter(|| r.move_side(black_box(Side::Right, Direction::Anticlockwise))));
    group.bench_function("move right twice", |b| b.iter(|| r.move_side(black_box(Side::Right, Direction::Twice))));
    group.bench_function("move right clockwise", |b| b.iter(|| r.move_side(black_box(Side::Right, Direction::Clockwise))));
    group.bench_function("move front anticlockwise", |b| b.iter(|| r.move_side(black_box(Side::Front, Direction::Anticlockwise))));
    group.bench_function("move front twice", |b| b.iter(|| r.move_side(black_box(Side::Front, Direction::Twice))));
    group.bench_function("move front clockwise", |b| b.iter(|| r.move_side(black_box(Side::Front, Direction::Clockwise))));
    group.bench_function("move back anticlockwise", |b| b.iter(|| r.move_side(black_box(Side::Back, Direction::Anticlockwise))));
    group.bench_function("move back twice", |b| b.iter(|| r.move_side(black_box(Side::Back, Direction::Twice))));
    group.bench_function("move back clockwise", |b| b.iter(|| r.move_side(black_box(Side::Back, Direction::Clockwise))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);