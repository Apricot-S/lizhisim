use criterion::{criterion_group, criterion_main, Criterion};
use random_hand::{create_rng, generate_random_pure_hand};

const SAMPLE_SIZE: usize = 100_000_000;
const NUM_RESAMPLE: usize = 2;

fn bench_empty(c: &mut Criterion) {
    let mut rng = create_rng();
    let mut group = c.benchmark_group("bench_empty");
    group.sample_size(SAMPLE_SIZE);
    group.nresamples(NUM_RESAMPLE);
    group.bench_function("bench_empty", |b| {
        b.iter(|| generate_random_pure_hand(&mut rng))
    });
    group.finish();
}

criterion_group!(empty, bench_empty);
criterion_main!(empty);
