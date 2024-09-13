use criterion::{criterion_group, criterion_main, Criterion};
use random_hand::{create_rng, generate_random_pure_hand};
use xiangting::calculate_replacement_number;

const SAMPLE_SIZE: usize = 100_000_000;

fn bench_empty(c: &mut Criterion) {
    let mut rng = create_rng();
    let mut group = c.benchmark_group("bench_empty");
    group.sample_size(SAMPLE_SIZE);
    group.bench_function("bench_empty", |b| {
        b.iter(|| generate_random_pure_hand(&mut rng))
    });
    group.finish();
}

fn bench_calculate(c: &mut Criterion) {
    let mut rng = create_rng();
    let mut group = c.benchmark_group("bench_calculate");
    group.sample_size(SAMPLE_SIZE);
    group.bench_function("bench_calculate", |b| {
        b.iter(|| calculate_replacement_number(&generate_random_pure_hand(&mut rng), &None))
    });
    group.finish();
}

criterion_group!(empty, bench_empty);
criterion_group!(calculate, bench_calculate);
criterion_main!(empty, calculate);
