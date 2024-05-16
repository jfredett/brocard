use crate::montgomery::{mod_inverse, mod_mult, gcd, Elt};

/// A Montgomery Space is a modulus `n` and a Montgomery constant `r` such that `r * r_inv - n *
/// n_inv = 1`. The Montgomery constant `r` is chosen such that `r > n` and `r` is a power of 2.
///
/// Montgomery Spaces can be 'entered' by multiplying a number `a` by `r` modulo `n` to get `a' =
/// ar mod n`. In a Montgomery Space, multiplication is done by multiplying two numbers `a` and `b`
/// together, then multiplying the result by `r_inv` modulo `n` to get `ab mod n`. This can be done
/// efficiently by the `redc` function. When `r` is chosen to be a power of 2, the `redc` function
/// can be implemented as a simple bit shift.
pub struct Space {
    pub r_exp: usize,
    pub r: u128,
    pub n: u128,
    pub r_inv: u128, 
    pub r_squared: u128,
    pub n_inv: u128,
    pub n_prime: u128
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

    /// REDC is the core of the Montgomery multiplication algorithm. It takes a number `a` and
    /// quickly reduces it modulo `n` by multiplying it by `n_prime` modulo `r` and then shifting
    /// right by `r_exp`. This is equivalent to multiplying by `r` modulo `n` and then reducing
    /// modulo `n`.
    ///
    /// This allows a _much_ faster modulo operation, since shifting is much cheaper than division.
    /// This scales up to multiprecision numbers, but we limit to 128b numbers here.
    pub fn redc(&self, a: u128) -> u128 {
        let little_m = mod_mult(a, self.n_prime, self.r);
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
    /// ```
    /// r         = 2^r_exp              // The Montgomery constant
    /// r_inv     = r^-1 mod n           // The modular inverse of r mod n
    /// r_squared = r^2 mod n            // Used for 'entering' the space
    /// n_inv     = n^-1 mod r           // The modular inverse of n mod r
    /// n_prime   = (r - n)^-1 mod r     // The modular inverse of -n mod r, used in `redc`
    /// ```
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
        // => n' = (r - n)^-1 mod r
        //
        let n_prime = mod_inverse(r - n, r).unwrap() % r;

        Space {
            r_exp,
            r,
            r_squared: mod_mult(r, r, n),
            r_inv,
            n,
            n_inv,
            n_prime
        }
    }
}