#![feature(iter_intersperse)]

use criterion::{criterion_group, criterion_main, Criterion};
use intersperse_perf::IteratorExt;

const ELEMENTS: u32 = 10000;

//
// *************************************************************************************************
//    These benchmarks create a sum of the elements in a slice, interspersed with 1s.
// *************************************************************************************************
//

pub fn bench_iters(c: &mut Criterion) {
    let elements = (0..ELEMENTS).collect::<Vec<_>>();
    let mut g = c.benchmark_group("Sum");

    g.bench_function("intersperse", |b| {
        b.iter(|| {
            let mut sum = 0;
            for v in elements.iter().intersperse(&1) {
                sum += v;
            }
            sum
        })
    });
    g.bench_function("my_intersperse", |b| {
        b.iter(|| {
            let mut sum = 0;
            for v in elements.iter().my_intersperse(&1) {
                sum += v;
            }
            sum
        })
    });
    g.bench_function("intersperse-opt", |b| {
        b.iter(|| {
            let mut sum = 0;
            for v in elements.iter().map(Some).intersperse(None) {
                if let Some(v) = v {
                    sum += v;
                } else {
                    sum += 1;
                }
            }
            sum
        })
    });
    g.bench_function("my_intersperse-opt", |b| {
        b.iter(|| {
            let mut sum = 0;
            for v in elements.iter().map(Some).my_intersperse(None) {
                if let Some(v) = v {
                    sum += v;
                } else {
                    sum += 1;
                }
            }
            sum
        })
    });
    g.bench_function("intersperse-opt-unwrap", |b| {
        b.iter(|| {
            let mut sum = 0;
            for v in elements.iter().map(Some).intersperse(None) {
                sum += v.unwrap_or(&1);
            }
            sum
        })
    });
    g.bench_function("my_intersperse-opt-unwrap", |b| {
        b.iter(|| {
            let mut sum = 0;
            for v in elements.iter().map(Some).my_intersperse(None) {
                sum += v.unwrap_or(&1);
            }
            sum
        })
    });
    g.bench_function("intersperse-with", |b| {
        b.iter(|| {
            let mut sum = 0;
            for v in elements.iter().intersperse_with(|| &1) {
                sum += v;
            }
            sum
        })
    });
    g.bench_function("my_intersperse-with", |b| {
        b.iter(|| {
            let mut sum = 0;
            for v in elements.iter().my_intersperse_with(|| &1) {
                sum += v;
            }
            sum
        })
    });
    g.bench_function("intersperse-with-opt", |b| {
        b.iter(|| {
            let mut sum = 0;
            for v in elements.iter().map(Some).intersperse_with(|| None) {
                if let Some(v) = v {
                    sum += v;
                } else {
                    sum += 1;
                }
            }
            sum
        })
    });
    g.bench_function("my_intersperse-with-opt", |b| {
        b.iter(|| {
            let mut sum = 0;
            for v in elements.iter().map(Some).my_intersperse_with(|| None) {
                if let Some(v) = v {
                    sum += v;
                } else {
                    sum += 1;
                }
            }
            sum
        })
    });
    g.bench_function("intersperse-with-opt-unwrap", |b| {
        b.iter(|| {
            let mut sum = 0;
            for v in elements.iter().map(Some).intersperse_with(|| None) {
                sum += v.unwrap_or(&1);
            }
            sum
        })
    });
    g.bench_function("my_intersperse-with-opt-unwrap", |b| {
        b.iter(|| {
            let mut sum = 0;
            for v in elements.iter().map(Some).my_intersperse_with(|| None) {
                sum += v.unwrap_or(&1);
            }
            sum
        })
    });

    g.finish();
}

criterion_group!(benches, bench_iters);
criterion_main!(benches);
