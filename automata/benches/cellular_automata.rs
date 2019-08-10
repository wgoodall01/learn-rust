#[macro_use]
extern crate criterion;

use automata::ca::*;
use criterion::{black_box, Criterion};

fn nth_layer(n: usize) -> Vec<bool> {
    iter_layers(30).skip(n).next().unwrap()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "nth_layer",
        |b, n| b.iter(|| nth_layer(black_box(*n))),
        vec![10, 50, 200],
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
