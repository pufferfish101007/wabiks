use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wabiks::rubiks::{Rubiks, Side, Direction};
use Direction::*;
use Side::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("benches");
    let r  = Rubiks::solved();
    group.bench_function("move up anticlockwise", |b| b.iter(|| r.move_side(black_box(Side::Up), black_box(Direction::Anticlockwise))));
    group.bench_function("move up twice", |b| b.iter(|| r.move_side(black_box(Side::Up), black_box(Direction::Twice))));
    group.bench_function("move up clockwise", |b| b.iter(|| r.move_side(black_box(Side::Up), black_box(Direction::Clockwise))));
    group.bench_function("move down anticlockwise", |b| b.iter(|| r.move_side(black_box(Side::Down), black_box(Direction::Anticlockwise))));
    group.bench_function("move down twice", |b| b.iter(|| r.move_side(black_box(Side::Down), black_box(Direction::Twice))));
    group.bench_function("move down clockwise", |b| b.iter(|| r.move_side(black_box(Side::Down), black_box(Direction::Clockwise))));
    group.bench_function("move left anticlockwise", |b| b.iter(|| r.move_side(black_box(Side::Left), black_box(Direction::Anticlockwise))));
    group.bench_function("move left twice", |b| b.iter(|| r.move_side(black_box(Side::Left), black_box(Direction::Twice))));
    group.bench_function("move left clockwise", |b| b.iter(|| r.move_side(black_box(Side::Left), black_box(Direction::Clockwise))));
    group.bench_function("move right anticlockwise", |b| b.iter(|| r.move_side(black_box(Side::Right), black_box(Direction::Anticlockwise))));
    group.bench_function("move right twice", |b| b.iter(|| r.move_side(black_box(Side::Right), black_box(Direction::Twice))));
    group.bench_function("move right clockwise", |b| b.iter(|| r.move_side(black_box(Side::Right), black_box(Direction::Clockwise))));
    group.bench_function("move front anticlockwise", |b| b.iter(|| r.move_side(black_box(Side::Front), black_box(Direction::Anticlockwise))));
    group.bench_function("move front twice", |b| b.iter(|| r.move_side(black_box(Side::Front), black_box(Direction::Twice))));
    group.bench_function("move front clockwise", |b| b.iter(|| r.move_side(black_box(Side::Front), black_box(Direction::Clockwise))));
    group.bench_function("move back anticlockwise", |b| b.iter(|| r.move_side(black_box(Side::Back), black_box(Direction::Anticlockwise))));
    group.bench_function("move back twice", |b| b.iter(|| r.move_side(black_box(Side::Back), black_box(Direction::Twice))));
    group.bench_function("move back clockwise", |b| b.iter(|| r.move_side(black_box(Side::Back), black_box(Direction::Clockwise))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);