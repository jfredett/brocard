#![feature(custom_test_frameworks)]
#![test_runner(criterion::runner)]

use std::env;
use rand::Rng;

pub use criterion::{black_box, Criterion};
pub use criterion_macro::criterion;

pub use quickcheck::{Arbitrary, Gen};

pub use brocard::montgomery::{Elt, Space, LegendreSymbol};
pub use brocard::prime::is_prime;

mod montgomery_bench;
mod legendre_bench;
mod prime_bench;


fn config() -> Criterion {
    Criterion::default()
        .sample_size(5000)
        .measurement_time(std::time::Duration::from_secs(120))
        .warm_up_time(std::time::Duration::from_secs(10))
}

/// This measurement serves as a reference for 'the fastest possible thing' this machine can do.
/// It is simply adding two constant f64 values inside a black box. It's performance should
/// basically be 1 operation + any overhead, giving some context to the other benchmarks.
#[criterion(config())]
pub fn reference(c: &mut Criterion) {
    c.bench_function("Reference Measurement", |b| b.iter(||
        black_box(2.0f64 + 2.0f64)
    ));
}

pub fn get_seed() -> u64 {
    match env::var("BENCH_SEED") {
        Ok(seed_str) => seed_str.parse().unwrap_or_else(|_| rand::thread_rng().gen()),
        Err(_) => rand::thread_rng().gen(),
    }
}
