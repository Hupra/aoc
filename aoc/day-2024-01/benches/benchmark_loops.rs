use aoc_lib::init;
use criterion::{criterion_group, criterion_main, Criterion};
use day_2024_01::{loop_match, while_if, while_let, while_match};

fn ini() -> (Vec<usize>, Vec<usize>) {
    let lines = init(2024, 1);
    let mut a: Vec<usize> = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap()
        })
        .collect();

    let mut b: Vec<usize> = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap()
        })
        .collect();

    a.sort();
    b.sort();

    (a, b)
}

fn benchmark_loop_match(c: &mut Criterion) {
    let (a, b) = ini();
    c.bench_function("loop_match", |bench| bench.iter(|| loop_match(&a, &b)));
}

fn benchmark_while_if(c: &mut Criterion) {
    let (a, b) = ini();
    c.bench_function("while_if", |bench| bench.iter(|| while_if(&a, &b)));
}

fn benchmark_while_let(c: &mut Criterion) {
    let (a, b) = ini();
    c.bench_function("while_let", |bench| bench.iter(|| while_let(&a, &b)));
}

fn benchmark_while_match(c: &mut Criterion) {
    let (a, b) = ini();
    c.bench_function("while_match", |bench| bench.iter(|| while_match(&a, &b)));
}

fn custom_criterion() -> Criterion {
    Criterion::default().measurement_time(std::time::Duration::from_secs(15)) // Extend measurement time (default is 5s)
}
criterion_group! {
    name = benches;
    config = custom_criterion(); // Apply the custom configuration
    targets = benchmark_while_if, benchmark_loop_match, benchmark_while_let, benchmark_while_match
}

criterion_main!(benches);
