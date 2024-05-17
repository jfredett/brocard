pub mod space;
pub mod elt;


pub use space::{LegendreSymbol, Space};
pub use elt::Elt;

#[cfg(test)]
pub mod test_case;

#[cfg(test)]
pub use test_case::TestCase;

// TODO: This almost certainly exists somewhere already
pub fn gcd(a: u128, b: u128) -> u128 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn mod_inverse(n: u128, r: u128) -> Option<u128> {
    let mut t = 0u128;
    let mut new_t = 1u128;
    let mut r = r;
    let mut new_r = n;

    while new_r != 0 {
        let quotient = r / new_r;
        // wrapping_sub works by wrapping around on overflow, so we don't need to check for
        // negative values
        t = t.wrapping_sub(quotient.wrapping_mul(new_t));
        std::mem::swap(&mut t, &mut new_t);
        r = r.wrapping_sub(quotient.wrapping_mul(new_r));
        std::mem::swap(&mut r, &mut new_r);
    }

    if r > 1 { return None; }

    Some(t)
}

pub fn mod_mult(a: u128, b: u128, n: u128) -> u128 {
    let mut result = 0;
    let mut a = a % n;
    let mut b = b % n;
    while b > 0 {
        if b & 1 == 1 {
            result = (result + a) % n;
        }
        a = (a << 1) % n;
        b = b >> 1;
    }
    result
}

pub fn mod_inv(a: u128, n: u128) -> u128 {
    let (x, _) = extended_gcd(a, n);
    (x % n + n) % n
}

pub fn mod_exp(a: u128, k: u128, n: u128) -> u128 {
    let mut a = a;
    let mut k = k;
    let mut result = 1;

    while k > 0 {
        if k & 1 == 1 {
            result = mod_mult(result, a, n);
        }
        a = mod_mult(a, a, n);
        k >>= 1;
    }

    result
}

pub fn extended_gcd(a: u128, b: u128) -> (u128, u128) {
    let mut s = 0;
    let mut old_s = 1;
    let mut t = 1;
    let mut old_t = 0;
    let mut r = b;
    let mut old_r = a;

    while r != 0 {
        let quotient = old_r / r;
        let temp = r;
        r = old_r - quotient * r;
        old_r = temp;

        let temp = s;
        s = old_s - quotient * s;
        old_s = temp;

        let temp = t;
        t = old_t - quotient * t;
        old_t = temp;
    }

    (old_s, old_t)
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::montgomery::test_case::TestCase;

    mod unit {
        use super::*;

        mod montgomery {
            use super::*;

            #[test]
            fn wikipedia_example() {
                let space = Space::new(17, 8);
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
                    let space = Space::new(21, 8);
                    let a = space.enter(7);

                    assert_eq!(
                        space.legendre(a),
                        LegendreSymbol::Divisor
                    );
                }

                #[test]
                fn legendre_showing_non_residue() {
                    let space = Space::new(21, 8);
                    let a = space.enter(11);

                    assert_eq!(
                        space.legendre(a),
                        LegendreSymbol::Nonresidue
                    );
                }

                #[test]
                fn legendre_showing_quadratic_residue() {
                    let space = Space::new(21, 8);
                    let a = space.enter(5);

                    assert_eq!(
                        space.legendre(a),
                        LegendreSymbol::Residue
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

        #[quickcheck]
        fn montgomery_add_is_naive_add(tc: TestCase) -> bool {
            let TestCase {a, b, n, r_exp} = tc;

            let naive = a.wrapping_add(b) % n;

            let space = Space::new(n, r_exp);
            let a = space.enter(a);
            let b = space.enter(b);
            let montgomery = (a + b).exit();

            naive == montgomery
        }

        #[quickcheck]
        fn montgomery_mul_is_naive_mul(tc: TestCase) -> bool {
            let TestCase {a, b, n, r_exp} = tc;

            let naive = mod_mult(a, b, n);

            let space = Space::new(n, r_exp);
            let a = space.enter(a);
            let b = space.enter(b);
            let montgomery = (a * b).exit();

            naive == montgomery
        }

        #[quickcheck]
        fn montgomery_sub_is_naive_sub(tc: TestCase) -> bool {
            let TestCase {a, b, n, r_exp} = tc;

            let naive = (a + (n - (b % n))) % n;

            let space = Space::new(n, r_exp);
            let a = space.enter(a);
            let b = space.enter(b);
            let montgomery = (a - b).exit();

            naive == montgomery
        }
    }
}
