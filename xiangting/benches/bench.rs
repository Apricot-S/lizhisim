use criterion::{criterion_group, criterion_main, Criterion};
use random_hand::{create_rng, generate_random_pure_hand};
use xiangting::calculate_replacement_number;

const NUM_HAND: usize = 100_000_000;
const SAMPLE_SIZE: usize = 10_000;
const NUM_RESAMPLE: usize = 1_000;

fn bench_xiangting(c: &mut Criterion) {
    let mut rng = create_rng();
    let hands: Vec<_> = (0..NUM_HAND)
        .map(|_| generate_random_pure_hand(&mut rng))
        .collect();

    let mut group = c.benchmark_group("bench_xiangting");
    group.sample_size(SAMPLE_SIZE);
    group.nresamples(NUM_RESAMPLE);
    group.bench_function("bench_xiangting", |b| {
        let mut hand = hands.iter();
        b.iter(|| calculate_replacement_number(hand.next().unwrap(), &None))
    });
    group.finish();
}

criterion_group!(bench, bench_xiangting);
criterion_main!(bench);
