/// This calculates the legendre symbol of a random value based on the input seed using different
/// multiplication methods.

use super::*;
use criterion::{black_box, Criterion};
use rand::rngs::StdRng;
use rand::SeedableRng;

use brocard::math::legendre::LegendreSymbol;
use brocard::montgomery::Space;
use quickcheck::{Arbitrary, StdGen};

#[criterion(config())]
fn legendre_bench(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(get_seed());
    let mut gen = StdGen::new(&mut rng, 1000);

    let a = u128::arbitrary(&mut gen);
    let p = (1 << 61) - 1; // this is a large mersenne prime, it's handy because it's short to
                         // remember.


    // TODO: Refactor to benchmark group
    c.bench_function("Legendre Symbol, Montgomery Multiplication, Fixed R_EXP: 8", |b| b.iter(|| {
        let space = Space::<8>::new(p);
        let a = space.enter(a);

        black_box(
            space.legendre(a)
        );
    }));

    c.bench_function("Legendre Symbol, Montgomery Multiplication, Fixed R_EXP: 16", |b| b.iter(|| {
        let space = Space::<16>::new(p);
        let a = space.enter(a);

        black_box(
            space.legendre(a)
        );
    }));

    c.bench_function("Legendre Symbol, Montgomery Multiplication, Fixed R_EXP: 32", |b| b.iter(|| {
        let space = Space::<32>::new(p);
        let a = space.enter(a);

        black_box(
            space.legendre(a)
        );
    }));

    c.bench_function("Legendre Symbol, Montgomery Multiplication, Fixed R_EXP: 64", |b| b.iter(|| {
        let space = Space::<64>::new(p);
        let a = space.enter(a);

        black_box(
            space.legendre(a)
        );
    }));

    c.bench_function("Legendre Symbol, Naive Multiplication", |b| b.iter(|| {
        black_box(
            LegendreSymbol::naive_legendre(a, p)
        );
    }));
}
