// Copyright 2023 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>
use criterion::{criterion_group, criterion_main, Criterion};
use ord_list_set;
use ord_list_set_alt::*;

pub fn from_benchmark(c: &mut Criterion) {
    let data = ["a", "b", "c", "g", "e", "f"];

    let mut group = c.benchmark_group("OrdListSet: From([T])");
    group.bench_function("ord_list_set::OrdListSet", |b| {
        b.iter(|| {
            let _result = ord_list_set::OrdListSet::<&str>::from(data);
        })
    });
    group.bench_function("OrdListSet", |b| {
        b.iter(|| {
            let _result = OrdListSet::<&str>::from(data);
        })
    });
    group.finish();
}

pub fn collect_benchmark(c: &mut Criterion) {
    let data = [
        "a", "b", "c", "g", "e", "f", "h", "i", "j", "k", "l", "m", "n", "o", "p",
    ];
    let orig_ord_list_set = ord_list_set::OrdListSet::from(data);
    let ord_list_set = OrdListSet::from(data);

    let mut group = c.benchmark_group("OrdListSet: iter().collect()");
    group.bench_function("ord_list_set::OrdListSet", |b| {
        b.iter(|| {
            let _result = orig_ord_list_set.iter().collect::<Vec<_>>();
        })
    });
    group.bench_function("OrdListSet", |b| {
        b.iter(|| {
            let _result = ord_list_set.iter().collect::<Vec<_>>();
        })
    });
    group.finish();
}

pub fn sum_benchmark(c: &mut Criterion) {
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let orig_ord_list_set = ord_list_set::OrdListSet::from(data);
    let ord_list_set = OrdListSet::from(data);

    let mut group = c.benchmark_group("OrdListSet: iter().sum()");
    group.bench_function("ord_list_set::OrdListSet", |b| {
        b.iter(|| {
            let _result = orig_ord_list_set.iter().sum::<i32>();
        })
    });
    group.bench_function("OrdListSet", |b| {
        b.iter(|| {
            let _result = ord_list_set.iter().sum::<i32>();
        })
    });
    group.finish();
}

pub fn next_benchmark(c: &mut Criterion) {
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let orig_ord_list_set = ord_list_set::OrdListSet::from(data);
    let ord_list_set = OrdListSet::from(data);

    let mut group = c.benchmark_group("OrdListSetIter::next()");
    group.bench_function("ord_list_set::OrdListSet", |b| {
        b.iter(|| {
            let mut sum = 0;
            let mut iter = orig_ord_list_set.iter();
            while let Some(i) = iter.next() {
                sum += i;
            }
        })
    });
    group.bench_function("OrdListSet", |b| {
        b.iter(|| {
            let mut sum = 0;
            let mut iter = ord_list_set.iter();
            while let Some(i) = iter.next() {
                sum += i;
            }
        })
    });
    group.finish();
}

pub fn difference_benchmark(c: &mut Criterion) {
    let data1 = ["a", "b", "c", "g", "e", "f"];
    let data2 = ["c", "f", "i", "l"];
    let orig_ord_list_set1 = ord_list_set::OrdListSet::from(data1);
    let orig_ord_list_set2 = ord_list_set::OrdListSet::from(data2);
    let ord_list_set1 = OrdListSet::from(data1);
    let ord_list_set2 = OrdListSet::from(data2);

    let mut group = c.benchmark_group("OrdListSet: Difference");
    group.bench_function("ord_list_set::OrdListSet: '-' operator'", |b| {
        b.iter(|| {
            let _result: ord_list_set::OrdListSet<&str> = &orig_ord_list_set1 - &orig_ord_list_set2;
        })
    });
    group.bench_function(
        "ord_list_set::OrdListSet: .difference().cloned().collect()",
        |b| {
            b.iter(|| {
                let _result: ord_list_set::OrdListSet<&str> = orig_ord_list_set1
                    .difference(&orig_ord_list_set2)
                    .cloned()
                    .collect();
            })
        },
    );
    group.bench_function("OrdListSet: '-' operator'", |b| {
        b.iter(|| {
            let _result: OrdListSet<&str> = &ord_list_set1 - &ord_list_set2;
        })
    });
    group.bench_function("OrdListSet: .difference().cloned().collect()", |b| {
        b.iter(|| {
            let _result: OrdListSet<&str> =
                ord_list_set1.difference(&ord_list_set2).cloned().collect();
        })
    });
    group.finish();
}

pub fn union_benchmark(c: &mut Criterion) {
    let data1 = ["a", "b", "c", "g", "e", "f"];
    let data2 = ["c", "f", "i", "l"];
    let orig_ord_list_set1 = ord_list_set::OrdListSet::from(data1);
    let orig_ord_list_set2 = ord_list_set::OrdListSet::from(data2);
    let ord_list_set1 = OrdListSet::from(data1);
    let ord_list_set2 = OrdListSet::from(data2);

    let mut group = c.benchmark_group("OrdListSet: Union");
    group.bench_function("ord_list_set::OrdListSet: '|' operator'", |b| {
        b.iter(|| {
            let _result: ord_list_set::OrdListSet<&str> = &orig_ord_list_set1 | &orig_ord_list_set2;
        })
    });
    group.bench_function(
        "ord_list_set::OrdListSet: .union().cloned().collect()",
        |b| {
            b.iter(|| {
                let _result: ord_list_set::OrdListSet<&str> = orig_ord_list_set1
                    .union(&orig_ord_list_set2)
                    .cloned()
                    .collect();
            })
        },
    );
    group.bench_function("OrdListSet: '|' operator'", |b| {
        b.iter(|| {
            let _result: OrdListSet<&str> = &ord_list_set1 | &ord_list_set2;
        })
    });
    group.bench_function("OrdListSet: .union().cloned().collect()", |b| {
        b.iter(|| {
            let _result: OrdListSet<&str> = ord_list_set1.union(&ord_list_set2).cloned().collect();
        })
    });
    group.finish();
}

criterion_group!(
    benches,
    from_benchmark,
    collect_benchmark,
    sum_benchmark,
    next_benchmark,
    difference_benchmark,
    union_benchmark,
);
criterion_main!(benches);
