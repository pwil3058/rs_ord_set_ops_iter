use criterion::{criterion_group, criterion_main, Criterion};
use ord_set_iter_set_ops::*;
use std::collections::BTreeSet;

pub fn peekable_benchmark(c: &mut Criterion) {
    let set = BTreeSet::<&str>::from(["a", "b", "c", "g", "e", "f"]);

    let mut group = c.benchmark_group("OSISO: Peekable overhead");
    group.bench_function("BTree.iter().collect()", |b| {
        b.iter(|| {
            let _result = set.iter().collect::<Vec<_>>();
        })
    });
    group.bench_function("BTree.iter().seekable().collect()", |b| {
        b.iter(|| {
            let _result = set.iter().peekable().collect::<Vec<_>>();
        })
    });
    group.finish();
}

pub fn union_benchmark(c: &mut Criterion) {
    use ord_set_iter_set_ops::*;
    let set1 = BTreeSet::<&str>::from(["a", "b", "c", "g", "e", "f"]);
    let set2 = BTreeSet::<&str>::from(["c", "f", "i", "l"]);

    let mut group = c.benchmark_group("OSISO: Union");
    group.bench_function("using '|' operator", |b| {
        b.iter(|| {
            let _result = &set1 | &set2;
        })
    });
    group.bench_function("union().cloned().collect()", |b| {
        b.iter(|| {
            let _result = set1.union(&set2).cloned().collect::<Vec<_>>();
        })
    });
    group.bench_function("oso_union().clone().collect()", |b| {
        b.iter(|| {
            let _result = set1.oso_union(&set2).cloned().collect::<Vec<_>>();
        })
    });
    group.finish();
}

pub fn intersection_benchmark(c: &mut Criterion) {
    let set1: BTreeSet<&str> = ["a", "b", "c", "g", "e", "f"].iter().cloned().collect();
    let set2: BTreeSet<&str> = ["c", "f", "i", "l"].iter().cloned().collect();

    let mut group = c.benchmark_group("OSISO: Intersection");
    group.bench_function("using '&' operator", |b| {
        b.iter(|| {
            let _result = &set1 & &set2;
        })
    });
    group.bench_function("intersection.cloned().collect()", |b| {
        b.iter(|| {
            let _result = set1.intersection(&set2).cloned().collect::<Vec<_>>();
        })
    });
    group.bench_function("oso_intersection.cloned().collect()", |b| {
        b.iter(|| {
            let _result = set1.oso_intersection(&set2).cloned().collect::<Vec<_>>();
        })
    });
    group.finish();
}

// pub fn difference_benchmark(c: &mut Criterion) {
//     let set1: BTreeSet<&str> = ["a", "b", "c", "g", "e", "f"].iter().cloned().collect();
//     let set2: BTreeSet<&str> = ["c", "f", "i", "l"].iter().cloned().collect();
//
//     let mut group = c.benchmark_group("Difference 8/2");
//     group.bench_function("native", |b| {
//         b.iter(|| {
//             let _result: BTreeSet<&str> = set1.difference(&set2).cloned().collect();
//         })
//     });
//     group.bench_function("oso", |b| {
//         b.iter(|| {
//             let _result: BTreeSet<&str> = set1.oso_difference(&set2).cloned().collect();
//         })
//     });
//     group.finish();
// }
//
// pub fn symmetric_difference_benchmark(c: &mut Criterion) {
//     let set1: BTreeSet<&str> = ["a", "b", "c", "g", "e", "f"].iter().cloned().collect();
//     let set2: BTreeSet<&str> = ["c", "f", "i", "l"].iter().cloned().collect();
//
//     let mut group = c.benchmark_group("Symmetric Difference 8/2");
//     group.bench_function("native", |b| {
//         b.iter(|| {
//             let _result: BTreeSet<&str> = set1.symmetric_difference(&set2).cloned().collect();
//         })
//     });
//     group.bench_function("oso", |b| {
//         b.iter(|| {
//             let _result: BTreeSet<&str> = set1.oso_symmetric_difference(&set2).cloned().collect();
//         })
//     });
//     group.finish();
// }

pub fn expression_benchmark(c: &mut Criterion) {
    let set1: BTreeSet<&str> = ["a", "b", "c", "g", "e", "f"].iter().cloned().collect();
    let set2: BTreeSet<&str> = ["c", "f", "i", "l"].iter().cloned().collect();
    let set3: BTreeSet<&str> = ["c", "g", "a", "l"].iter().cloned().collect();

    let mut group = c.benchmark_group("OSISO: Expression");
    group.bench_function("native", |b| {
        b.iter(|| {
            let _result: BTreeSet<&str> = &(&set1 | &set2) & &set3;
        })
    });
    group.bench_function("oso", |b| {
        b.iter(|| {
            let _result: BTreeSet<&str> =
                set1.oso_union(&set2).intersection(set3.oso_iter()).into();
        })
    });
    group.finish();
}
//
// pub fn overhead_benchmark(c: &mut Criterion) {
//     let set1: BTreeSet<&str> = ["a", "b", "c", "g", "e", "f"].iter().cloned().collect();
//
//     let mut group = c.benchmark_group("Overhead 6");
//     group.bench_function("native", |b| {
//         b.iter(|| {
//             let _result: BTreeSet<&str> = set1.iter().cloned().collect();
//         })
//     });
//     group.bench_function("peekable", |b| {
//         b.iter(|| {
//             let _result: BTreeSet<&str> = set1.iter().peekable().cloned().collect();
//         })
//     });
//     group.bench_function("oso", |b| {
//         b.iter(|| {
//             let _result: BTreeSet<&str> = set1.oso_iter().cloned().collect();
//         })
//     });
//     group.finish();
// }

criterion_group!(
    benches,
    peekable_benchmark,
    union_benchmark,
    intersection_benchmark,
    // difference_benchmark,
    // symmetric_difference_benchmark,
    expression_benchmark,
    // overhead_benchmark,
);
criterion_main!(benches);
