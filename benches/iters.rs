#![feature(iter_intersperse)]

use criterion::{criterion_group, criterion_main, Criterion};
use intersperse_perf::IteratorExt;

const ELEMENTS: u32 = 10000;

//
// *************************************************************************************************
//    These benchmarks create a sum of the elements in a slice, interspersed with 1s.
// *************************************************************************************************
//

macro_rules! all_benchmarks {
    ($name:literal, $intersperse:ident, $intersperse_with:ident, $elements:expr, $g:expr) => {
        $g.bench_function(concat!("iter ", $name), |b| {
            b.iter(|| {
                let mut sum = 0;
                for v in $elements.iter().$intersperse(&1) {
                    sum += v;
                }
                sum
            })
        });
        $g.bench_function(concat!("opt ", $name), |b| {
            b.iter(|| {
                let mut sum = 0;
                for v in $elements.iter().map(Some).$intersperse(None) {
                    if let Some(v) = v {
                        sum += v;
                    } else {
                        sum += 1;
                    }
                }
                sum
            })
        });
        $g.bench_function(concat!("opt-unwrap ", $name), |b| {
            b.iter(|| {
                let mut sum = 0;
                for v in $elements.iter().map(Some).$intersperse(None) {
                    sum += v.unwrap_or(&1);
                }
                sum
            })
        });
        $g.bench_function(concat!("with ", $name), |b| {
            b.iter(|| {
                let mut sum = 0;
                for v in $elements.iter().$intersperse_with(|| &1) {
                    sum += v;
                }
                sum
            })
        });
        $g.bench_function(concat!("with-opt ", $name), |b| {
            b.iter(|| {
                let mut sum = 0;
                for v in $elements.iter().map(Some).$intersperse_with(|| None) {
                    if let Some(v) = v {
                        sum += v;
                    } else {
                        sum += 1;
                    }
                }
                sum
            })
        });
        $g.bench_function(concat!("with-opt-unwrap ", $name), |b| {
            b.iter(|| {
                let mut sum = 0;
                for v in $elements.iter().map(Some).$intersperse_with(|| None) {
                    sum += v.unwrap_or(&1);
                }
                sum
            })
        });
    };
}

pub fn bench_iters(c: &mut Criterion) {
    let elements = (0..ELEMENTS).collect::<Vec<_>>();
    let mut g = c.benchmark_group("_");

    all_benchmarks!("000", intersperse, intersperse_with, elements, g);
    all_benchmarks!("111", intersperse1, intersperse1_with, elements, g);
    all_benchmarks!("222", intersperse2, intersperse2_with, elements, g);

    g.finish();
}

criterion_group!(benches, bench_iters);
criterion_main!(benches);
