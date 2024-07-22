use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode_rs::medium::p15_three_sum::{three_sum, three_sum_v2, three_sum_v3};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("three_sum", |b| {
        b.iter(|| three_sum(black_box(vec![-1, 0, 1, 2, -1, -4])))
    });
    c.bench_function("three_sum_v2", |b| {
        b.iter(|| three_sum_v2(black_box(vec![-1, 0, 1, 2, -1, -4])))
    });
    c.bench_function("three_sum_v3", |b| {
        b.iter(|| three_sum_v3(black_box(vec![-1, 0, 1, 2, -1, -4])))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
