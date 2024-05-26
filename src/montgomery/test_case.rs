use crate::math::gcd;
use quickcheck::{Arbitrary, Gen};

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
