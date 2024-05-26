use super::*;
use criterion::{BenchmarkId, Criterion};
use rand::rngs::StdRng;
use rand::SeedableRng;

use brocard::math::{gcd, mod_mult};
use quickcheck::{Arbitrary, StdGen, Gen};

// FIXME: This sucks, copied from test_case
#[derive(Clone, Copy, Debug)]
pub struct TestCase {
    pub(crate) a: u128,
    pub(crate) b: u128,
    pub(crate) n: u128,
    pub(crate) r_exp: usize
}

impl Arbitrary for TestCase {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        // r_exp is in [1,127] to avoid overflows
        let r_exp = 1 + usize::arbitrary(g) % 126;
        let r = 1 << r_exp;

        // We'll just hunt till we find a coprime `n`, should be fast, any odd number will
        // do. We also need `n < r`, so we can just examine `n mod r` to ensure this.
        let mut n = u128::arbitrary(g) % r;
        while gcd(n, r) != 1 {
            n = u128::arbitrary(g) % r;
        }

        // Constrain a and b to [0,n-1] for convenience
        let a = u128::arbitrary(g) % n;
        let b = u128::arbitrary(g) % n;

        TestCase { a, b, n, r_exp }
    }
}

#[criterion(config())]
fn montgomery_multiplication_fixed_r_exp(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(get_seed());
    let mut gen = StdGen::new(&mut rng, 1000);

    let mut group = c.benchmark_group("Montgomery Multiplication with Fixed Exponent");

    // FIXME: There is probably a better way to do this.
    group.bench_with_input(BenchmarkId::new("R_EXP = ", 8), &8, |bench, _r_exp| {
        bench.iter(|| {
            let TestCase { a, b, n, r_exp: _ } = TestCase::arbitrary(&mut gen);

            let space = Space::<8>::new(n);

            black_box(
                (space.enter(a) * space.enter(b)).exit()
            );
        });
    });
    group.bench_with_input(BenchmarkId::new("R_EXP = ", 16), &16, |bench, _r_exp| {
        bench.iter(|| {
            let TestCase { a, b, n, r_exp: _ } = TestCase::arbitrary(&mut gen);

            let space = Space::<16>::new(n);

            black_box(
                (space.enter(a) * space.enter(b)).exit()
            );
        });
    });
    group.bench_with_input(BenchmarkId::new("R_EXP = ", 32), &32, |bench, _r_exp| {
        bench.iter(|| {
            let TestCase { a, b, n, r_exp: _ } = TestCase::arbitrary(&mut gen);

            let space = Space::<32>::new(n);

            black_box(
                (space.enter(a) * space.enter(b)).exit()
            );
        });
    });
    group.bench_with_input(BenchmarkId::new("R_EXP = ", 64), &64, |bench, _r_exp| {
        bench.iter(|| {
            let TestCase { a, b, n, r_exp: _ } = TestCase::arbitrary(&mut gen);

            let space = Space::<64>::new(n);

            black_box(
                (space.enter(a) * space.enter(b)).exit()
            );
        });
    });
    group.finish();

}

#[criterion(config())]
fn naive_multiplication(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(get_seed());
    let mut gen = StdGen::new(&mut rng, 1000);
    c.bench_function("Naive modular multiplication", |b| b.iter(|| {
        let TestCase { a, b, n, r_exp: _ } = TestCase::arbitrary(&mut gen);
        black_box(
            mod_mult(a, b, n)
        );
    }));
}

