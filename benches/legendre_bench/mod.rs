/// This calculates the legendre symbol of a random value based on the input seed using different
/// multiplication methods.

use super::*;
use criterion::{black_box, BenchmarkId, Criterion};

use brocard::math::legendre::LegendreSymbol;
use brocard::montgomery::Space;

const RANGE : std::ops::Range<u128> = 10_000..20_000;
#[criterion(config())]
fn legendre_bench(c: &mut Criterion) {
    let p = (1 << 61) - 1; // this is a large mersenne prime, it's handy because it's short to
                         // remember.

    let mut group = c.benchmark_group("Legendre Symbol");



    // I feel like this offends all that is good and right in the world, and I am ashamed for it,
    // but I will not fix it because that seems exceptionally boring. I'm sorry.
    group.bench_with_input(BenchmarkId::new("Montgomery", "R_EXP = 8"), &8, |bench, _r_exp| {
        bench.iter(|| {
            for a in RANGE {
                let space = Space::<8>::new(p);
                let a = space.enter(a);

                black_box(
                    space.legendre(a)
                );
            }
        });
    });

    group.bench_with_input(BenchmarkId::new("Montgomery", "R_EXP = 16"), &16, |bench, _r_exp| {
        bench.iter(|| {
            for a in RANGE {
                let space = Space::<16>::new(p);
                let a = space.enter(a);

                black_box(
                    space.legendre(a)
                );
            }
        });
    });

    group.bench_with_input(BenchmarkId::new("Montgomery", "R_EXP = 32"), &32, |bench, _r_exp| {
        bench.iter(|| {
            for a in RANGE {
                let space = Space::<32>::new(p);
                let a = space.enter(a);

                black_box(
                    space.legendre(a)
                );
            }
        });
    });

    group.bench_with_input(BenchmarkId::new("Montgomery", "R_EXP = 64"), &"R_EXP = 64", |bench, _r_exp| {
        bench.iter(|| {
            for a in RANGE {
                let space = Space::<64>::new(p);
                let a = space.enter(a);

                black_box(
                    space.legendre(a)
                );
            }
        });
    });


    group.bench_with_input(BenchmarkId::new("Naive", 0), &0, |bench, _r_exp| {
        bench.iter(|| {
            for a in RANGE {
                black_box(
                    LegendreSymbol::naive_legendre(a, p)
                );
            }
        });
    });

    group.finish();
}
