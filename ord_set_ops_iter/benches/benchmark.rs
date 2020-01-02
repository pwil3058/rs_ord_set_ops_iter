use criterion::{criterion_group, criterion_main, BenchmarkId};
use ord_set_ops_iter::adapter::*;
use std::collections::BTreeSet;

pub fn union_benchmark(c: &mut Criterion) {
    let set1: BTreeSet<&str> = ["a", "b", "c", "g", "e", "f"].iter().cloned().collect();
    let set2: BTreeSet<&str> = ["c", "f", "i", "l"].iter().cloned().collect();

    let mut group = c.benchmark_group("Union 8/2");
    group.bench_function("native", |b| {
        b.iter(|| {
            let _result: BTreeSet<&str> = set1.union(&set2).cloned().collect();
        })
    });
    group.bench_function("oso", |b| {
        b.iter(|| {
            let _result: BTreeSet<&str> =
                set1.oso_union(&set2).cloned().collect::<BTreeSet<&str>>();
        })
    });
    group.finish();
}

pub fn intersection_benchmark(c: &mut Criterion) {
    let set1: BTreeSet<&str> = ["a", "b", "c", "g", "e", "f"].iter().cloned().collect();
    let set2: BTreeSet<&str> = ["c", "f", "i", "l"].iter().cloned().collect();

    let mut group = c.benchmark_group("Intersection 8/2");
    group.bench_function("native", |b| {
        b.iter(|| {
            let _result: BTreeSet<&str> = set1.intersection(&set2).cloned().collect();
        })
    });
    group.bench_function("oso", |b| {
        b.iter(|| {
            let _result: BTreeSet<&str> = set1.oso_intersection(&set2).cloned().collect();
        })
    });
    group.finish();
}

criterion_group!(benches, union_benchmark, intersection_benchmark);
criterion_main!(benches);
