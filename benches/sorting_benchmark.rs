use numrs::rsarray::*;
use rand;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_sum(c: &mut Criterion) {
    fn f(v: Vec<f64>) -> f64 {
        let a = rsarray { items: v };
        a.sum()
    }
    let arr: Vec<f64> = black_box((0..10000).map(|_| rand::random::<f64>()).collect());

    c.bench_function("min()", |b| b.iter(|| f(arr.clone())));
}

fn bench_min(c: &mut Criterion) {
    fn f(v: Vec<f64>) -> f64 {
        let a = rsarray { items: v };
        a.min()
    }
    let arr: Vec<f64> = black_box((0..10000).map(|_| rand::random::<f64>()).collect());

    c.bench_function("sorting algorithm", |b| b.iter(|| f(arr.clone())));
}

fn bench_max(c: &mut Criterion) {
    fn f(v: Vec<f64>) -> f64 {
        let a = rsarray { items: v };
        a.max()
    }
    let arr: Vec<f64> = black_box((0..10000).map(|_| rand::random::<f64>()).collect());

    c.bench_function("max()", |b| b.iter(|| f(arr.clone())));
}

criterion_group!(benches, bench_min, bench_max);
criterion_main!(benches);
