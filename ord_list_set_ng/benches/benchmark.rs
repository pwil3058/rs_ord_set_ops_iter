// Copyright 2023 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>
use criterion::{criterion_group, criterion_main, Criterion};
use ord_list_set_ng::*;
//use ord_set_iter_set_ops::SetOsoIter;
use std::collections::btree_set::*;
use std::iter::FromIterator;

pub fn from_benchmark(c: &mut Criterion) {
    let data = ["a", "b", "c", "g", "e", "f"];

    let mut group = c.benchmark_group("From([T]): OrdListSet vs BTreeSet");
    group.bench_function("BTreeSet", |b| {
        b.iter(|| {
            let _result = BTreeSet::<&str>::from(data);
        })
    });
    group.bench_function("OrdListSet", |b| {
        b.iter(|| {
            let _result = OrdListSet::<&str>::from(data);
        })
    });
    group.finish();
}

pub fn iter_benchmark(c: &mut Criterion) {
    let data = ["a", "b", "c", "g", "e", "f"];
    let btree_set = BTreeSet::from(data);
    let ord_list_set = OrdListSet::from(data);

    let mut group = c.benchmark_group("iter().collect(): OrdListSet vs BTreeSet");
    group.bench_function("BTreeSet", |b| {
        b.iter(|| {
            let _result = btree_set.iter().collect::<Vec<_>>();
        })
    });
    group.bench_function("OrdListSet", |b| {
        b.iter(|| {
            let _result = ord_list_set.iter().collect::<Vec<_>>();
        })
    });
    group.finish();
}

pub fn union_benchmark(c: &mut Criterion) {
    let data1 = ["a", "b", "c", "g", "e", "f"];
    let data2 = ["c", "f", "i", "l"];
    let btree_set1 = BTreeSet::from(data1);
    let btree_set2 = BTreeSet::from(data2);
    let ord_list_set1 = OrdListSet::from(data1);
    let ord_list_set2 = OrdListSet::from(data2);

    let mut group = c.benchmark_group("Union: OrdListSet vs BTreeSet");
    group.bench_function("BTreeSet: .union().cloned().collect()", |b| {
        b.iter(|| {
            let _result: BTreeSet<&str> = btree_set1.union(&btree_set2).cloned().collect();
        })
    });
    // group.bench_function("BTreeSet: .oso_union().into()", |b| {
    //     b.iter(|| {
    //         let _result: BTreeSet<&str> = btree_set1.oso_union(&btree_set2).into();
    //     })
    // });
    group.bench_function("BTreeSet: '|' operator'", |b| {
        b.iter(|| {
            let _result: BTreeSet<&str> = &btree_set1 | &btree_set2;
        })
    });
    group.bench_function("OrdListSet: .union().into()", |b| {
        b.iter(|| {
            let _result: OrdListSet<&str> = ord_list_set1.union(&ord_list_set2).into();
        })
    });
    group.bench_function("OrdListSet: .union().cloned().collect()", |b| {
        b.iter(|| {
            let _result: OrdListSet<&str> = ord_list_set1.union(&ord_list_set2).cloned().collect();
        })
    });
    group.bench_function("OrdListSet: '|' operator'", |b| {
        b.iter(|| {
            let _result: OrdListSet<&str> = &ord_list_set1 | &ord_list_set2;
        })
    });
    // group.bench_function("OrdListSet: .oso_union().into()", |b| {
    //     b.iter(|| {
    //         let _result: OrdListSet<&str> = ord_list_set1.oso_union(&ord_list_set2).into();
    //     })
    // });
    group.finish();
}

criterion_group!(benches, from_benchmark, iter_benchmark, union_benchmark,);
criterion_main!(benches);
