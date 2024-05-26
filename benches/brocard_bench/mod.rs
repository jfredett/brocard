// This benchmark will verify the first billion `n` for the Brocard problem.
// It will use different implementations of this crate, and collect performance
// data and timing.
//
// The intent is to identify the most efficient way to verify values are not solutions.
// In the eventual distribution of this across multiple machines, the underlying verifier will
// undergo iteration to improve it's efficiency; so having this benchmark available and run on the
// hardware that will eventually be attacking this will make it easy to see which implementation
// is the one to use.
use super::*;
use criterion::{Criterion, BenchmarkId};

use brocard::math::prime::segmented_seive;
use brocard::brocard::BrocardSpan;


#[criterion(config())]
fn brocard_test(c: &mut Criterion) {

    let mut group = c.benchmark_group("Brocard Span Solver Benchmark");

    let vals = vec![
        10_000,
        20_000,
        30_000
    ];

    for val in vals {
        let primes_vec = segmented_seive(val, val + 1_000);
        let mut primes = [0; 60];

        for i in 0..60 {
            primes[i] = primes_vec[i];
        }

        let span = BrocardSpan::new(2, val, primes);

        group.bench_with_input(BenchmarkId::from_parameter(val), &val, |bench, _val| {
            bench.iter(|| {
                span.solve();
            });
        });
    }
    group.finish();
}
