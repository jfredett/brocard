use crate::math::{
    mod_inverse, mod_mult, gcd,
    legendre::LegendreSymbol
};
use crate::montgomery::Elt;

/// A Montgomery Space is a modulus `n` and a Montgomery constant `r` such that `r * r_inv - n *
/// n_inv = 1`. The Montgomery constant `r` is chosen such that `r > n` and `r` is a power of 2.
///
/// Montgomery Spaces can be 'entered' by multiplying a number `a` by `r` modulo `n` to get `a' =
/// ar mod n`. In a Montgomery Space, multiplication is done by multiplying two numbers `a` and `b`
/// together, then multiplying the result by `r_inv` modulo `n` to get `ab mod n`. This can be done
/// efficiently by the `redc` function. When `r` is chosen to be a power of 2, the `redc` function
/// can be implemented as a simple bit shift.
#[derive(Debug, Clone)]
pub struct Space {
    pub r_exp: usize,
    pub r: u128,
    pub n: u128,
    pub r_inv: u128, 
    pub r_squared: u128, pub n_inv: u128,
    pub n_prime: u128,
    pub r_n_legendre: LegendreSymbol
}


impl PartialEq for Space {
    fn eq(&self, other: &Space) -> bool {
        self.n == other.n && self.r == other.r
    }
}

impl Space {
    /// Entering the Montgomery "Space" is the first step in the Montgomery multiplication algorithm.
    /// This converts a number `a` into `aR mod N`, where `R = 2^r_exp` and `N` is the modulus.
    pub fn enter(&self, a: u128) -> Elt {
        let val = self.redc(mod_mult(a, self.r_squared, self.n));

        Elt {
            val,
            space: self
        }
    }

    pub fn factorial(&self, n: u128) -> Elt {
        let mut result = self.enter(1);
        for i in 1..=n {
            result = result * self.enter(i as u128);
        }
        return result;
    }

    /// Calculates aRn via aRr * rRn, where rRn is precomputed and cached at creation time.
    pub fn legendre(&self, a: Elt) -> LegendreSymbol {
        let exp = (self.n - 1) >> 1;
        let result = a.exp(exp).exit();

        let a_r_sym = if result == self.n - 1 {
            LegendreSymbol::Nonresidue
        } else if result == 1 {
            LegendreSymbol::Residue
        } else {
            LegendreSymbol::Divisor
        };

        a_r_sym * self.r_n_legendre
    }


    /// REDC is the core of the Montgomery multiplication algorithm. It takes a number `a` and
    /// quickly reduces it modulo `n` by multiplying it by `n_prime` modulo `r` and then shifting
    /// right by `r_exp`. This is equivalent to multiplying by `r` modulo `n` and then reducing
    /// modulo `n`.
    ///
    /// This allows a _much_ faster modulo operation, since shifting is much cheaper than division.
    /// This scales up to multiprecision numbers, but we limit to 128b numbers here.
    pub fn redc(&self, a: u128) -> u128 {
        // k mod r, since r is a power of two, is just the `r_exp` least significant bits of k.
        // that can be calculated by `k & (r - 1)`. This is equivalent to `k % r` when `r` is a
        // power of two.
        let mod_r = self.r - 1;
        let little_m = ((a & mod_r) * self.n_prime) & mod_r; 
        let new_t = (a + (little_m * self.n)) >> self.r_exp;

        if new_t >= self.n {
            new_t - self.n
        } else {
            new_t
        }
    }

    /// n is the modulus, r_exp is the exponent of the Montgomery constant r = 2^r_exp.
    /// This function calculates all other relevant constants, in particular it calculates:
    ///
    /// r         = 2^r_exp              // The Montgomery constant
    /// r_inv     = r^-1 mod n           // The modular inverse of r mod n
    /// r_squared = r^2 mod n            // Used for 'entering' the space
    /// n_inv     = n^-1 mod r           // The modular inverse of n mod r
    /// n_prime   = (r - n)^-1 mod r     // The modular inverse of -n mod r, used in `redc`
    ///
    ///
    pub fn new(n: u128, r_exp: usize) -> Space {
        let r = 1 << r_exp;

        assert!(gcd(r, n) == 1);

        let r_inv = mod_inverse(r, n).unwrap() % n;
        let n_inv = mod_inverse(n, r).unwrap() % r;

        // n_prime is _not_ the modular inverse of n mod r. Since we're operating unsigned, we
        // can't rely on the extended GCD to calculate it, fortunately it's easy to recover without
        // needing to manage a sign bit.
        //
        //    rr^-1 - nn' = 1 mod r
        // => 0     - nn' = 1 mod r
        // => nn' = -1        mod r
        // => n(-n^-1) = 1    mod r
        // => n' = -n^-1      mod r
        // => n' = (r - n)^-1 mod r  // Since n < r, we know this will never underflow in the subtraction.
        //
        let n_prime = mod_inverse(r - n, r).unwrap() % r;

        // This is used in the speedup of the legendre symbol calculation in #legendre.
        // We have to use the naive calculation here, but we only do this once and cache it.
        // Then the legendre symbol can be calculated as (aRr * rRn). This is a speedup because
        // we're doing the expensive operation mod R, and R is really easy to divide by.
        let r_n_legendre = LegendreSymbol::naive_legendre(r, n);

        Space {
            r_exp,
            r,
            r_squared: mod_mult(r, r, n),
            r_inv,
            r_n_legendre,
            n,
            n_inv,
            n_prime
        }
    }


}
