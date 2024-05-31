pub mod space;
pub mod elt;


pub use space::Space;
pub use elt::Elt;

#[cfg(test)]
pub mod test_case;

#[cfg(test)]
pub use test_case::TestCase;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::montgomery::test_case::TestCase;
    use crate::math::legendre::LegendreSymbol;
    use crate::math::*;

    const R_EXP: usize = 64;

    mod unit {
        use super::*;


        mod montgomery {
            use super::*;

            #[test]
            fn wikipedia_example() {
                let space = Space::<{R_EXP}>::new(17);
                let a = space.enter(7);
                let b = space.enter(15);

                let c = a * b;
                assert_eq!(c.exit(), 3);
            }

        }

        mod legendre {
            use super::*;

            mod montgomery {
                use super::*;

                #[test]
                fn legendre_showing_divisor() {
                    let space = Space::<{R_EXP}>::new(7);
                    let a = space.enter(21);

                    assert_eq!(
                        space.legendre(a),
                        LegendreSymbol::naive_legendre(21, 7)
                    );
                }

                #[test]
                fn legendre_showing_non_residue() {
                    let space = Space::<{R_EXP}>::new(11);
                    let a = space.enter(21);

                    dbg!("nonresidue", &a, &space);

                    assert_eq!(
                        space.legendre(a),
                        LegendreSymbol::naive_legendre(21, 11)
                    );
                }

                #[test]
                fn legendre_showing_quadratic_residue() {
                    let space = Space::<{R_EXP}>::new(5);
                    let a = space.enter(21);

                    assert_eq!(
                        space.legendre(a),
                        LegendreSymbol::naive_legendre(21, 5)
                    );
                }
            }

            mod naive {
                use super::*;

                #[test]
                fn legendre_showing_divisor() {
                    assert_eq!(
                        LegendreSymbol::naive_legendre(21, 7),
                        LegendreSymbol::Divisor
                    );
                }

                #[test]
                fn legendre_showing_non_residue() {
                    assert_eq!(
                        LegendreSymbol::naive_legendre(21, 11),
                        LegendreSymbol::Nonresidue
                    );
                }

                #[test]
                fn legendre_showing_quadratic_residue() {
                    assert_eq!(
                        LegendreSymbol::naive_legendre(21, 5),
                        LegendreSymbol::Residue
                    );
                }
            }

        }
    }

    mod props {
        use super::*;


        mod legendre {
            use super::*;

            #[quickcheck]
            fn montgomery_legendre_is_naive_legendre(a: u128) -> bool {

                let n = (1 << 61) - 1; //A friendly Mersenne Prime Appears

                let naive = LegendreSymbol::naive_legendre(a, n);

                let space = Space::<{R_EXP}>::new(n);
                let a = space.enter(a);
                let montgomery = space.legendre(a);

                naive == montgomery
            }

        }

        mod factorial {
            use super::*;

            #[quickcheck]
            fn montgomery_factorial_is_naive_factorial(k: u8, n: u128) -> bool {
                let space = Space::<{R_EXP}>::new(n);
                let montgomery = space.factorial(k as u128);

                let naive = (1..=k.into()).fold(1, |acc, x| mod_mult(acc, x, n));

                naive == montgomery.exit()
            }
        }

        mod montgomery_ops {
            use super::*;

            const R_EXP: usize = 64;

            #[quickcheck]
            fn montgomery_add_is_naive_add(tc: TestCase) -> bool {
                let TestCase {a, b, n, r_exp: _} = tc;

                let naive = a.wrapping_add(b) % n;

                let space = Space::<{R_EXP}>::new(n);
                let a = space.enter(a);
                let b = space.enter(b);
                let montgomery = (a + b).exit();

                naive == montgomery
            }

            #[quickcheck]
            fn montgomery_mul_of_u128_is_correct(tc: TestCase) -> bool {
                let TestCase {a, b, n, r_exp: _} = tc;

                let space = Space::<{R_EXP}>::new(n);
                let x = space.enter(a);
                let y = space.enter(b);

                (x + b) == (x + y)
            }

            #[quickcheck]
            fn montgomery_mul_is_naive_mul(tc: TestCase) -> bool {
                let TestCase {a, b, n, r_exp: _} = tc;

                let naive = mod_mult(a, b, n);

                let space = Space::<{R_EXP}>::new(n);
                let a = space.enter(a);
                let b = space.enter(b);
                let montgomery = (a * b).exit();

                naive == montgomery
            }

            #[quickcheck]
            fn montgomery_add_of_u128_is_correct(tc: TestCase) -> bool {
                let TestCase {a, b, n, r_exp: _} = tc;

                let space = Space::<{R_EXP}>::new(n);
                let x = space.enter(a);
                let y = space.enter(b);

                (x * b) == (x * y)
            }

            #[quickcheck]
            fn montgomery_sub_is_naive_sub(tc: TestCase) -> bool {
                let TestCase {a, b, n, r_exp: _} = tc;

                let naive = (a + (n - (b % n))) % n;

                let space = Space::<{R_EXP}>::new(n);
                let a = space.enter(a);
                let b = space.enter(b);
                let montgomery = (a - b).exit();

                naive == montgomery
            }
        }
    }
}
