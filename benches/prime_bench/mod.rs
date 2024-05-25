use super::*;
use criterion::{Criterion, BenchmarkId};

use brocard::prime::{is_prime, segmented_seive};


/// This benchmark tests the MR implementation against 4 different values, two are primes, two are
/// composite (I think, I didn't check, but it's probably composite).
/// If, at some point a number reveals itself to be particularly 'bad', then I'll add it to the
/// list.
#[criterion(config())]
fn miller_rabin_test(c: &mut Criterion) {

    let mut group = c.benchmark_group("Miller-Rabin Primality Test");

    let vals : [u128; 4] = [
        63_018_038_201, // a large prime
        66000049 * 3331333, // the product of two primes
        8_675_309_867_530_999, // A fairly large number
        (1 << 61) - 1, // a _very_ large number
    ];

    for val in vals {
        group.bench_with_input(BenchmarkId::from_parameter(val), &val, |bench, val| {
            bench.iter(|| {
                is_prime(*val);
            });
        });
    }
    group.finish();
}

#[criterion(config())]
fn segmented_seive_test(c: &mut Criterion) {
    let mut group = c.benchmark_group("Segmented Seive");

    let spans : [u128; 3] = [
        1_000,
        10_000,
        100_000
    ];

    for span in spans {
        group.bench_with_input(BenchmarkId::from_parameter(span), &span, |bench, span| {
            bench.iter(|| {
                segmented_seive(1_000_000, 1_000_000 + span);
            });
        });
    }
    group.finish();
}
