use std::fmt;
use std::fmt::{Formatter,Debug};

fn gcd(a: u128, b: u128) -> u128 {
    if a > b { return gcd(b, a); }
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

fn mod_mult(a: u128, b: u128, m: u128) -> u128 {
    let mut ans = 0;
    let mut a = a % m;
    let mut b = b % m;
    while b > 0 {
        if b & 1 == 1 {
            ans = (ans + a) % m;
        }
        a = (a << 1) % m;
        b >>= 1;
    }
    ans
}

fn modular_exponent(mut n:u128 ,mut x:u128 , p:u128) -> u128 {
    let mut ans = 1;
    if x <= 0 { return 1; }
    loop {
        if x == 1 { return (ans * n) % p; }

        if x & 1 == 0 { 
            n = ( n * n ) % p;
            x >>= 1;
        } else { 
            ans = (ans*n) % p;
            x -= 1; 
        }
    }
}


fn mod_inverse(n: u128, r: u128) -> Option<u128> {
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

#[derive(Clone)]
struct MontgomeryForm {
    exp: isize,
    r: u128, 
    r_squared: u128,
    r_inv: u128,
    n: u128, 
    n_prime: u128
}

#[derive(Clone)]
struct MontgomeryElt {
    space: Box<MontgomeryForm>,
    val: u128
}

impl Debug for MontgomeryElt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "MontgomeryElt({})", self.val)
    }
}


impl MontgomeryForm {
    fn new(n: u128, exp: isize) -> Self {
        let r = 1 << exp;

        assert!(gcd(r, n) == 1);

        let r_inv = mod_inverse(r, n).unwrap() % n;

        // rr^-1 - nn' = 1 mod r =>
        // 0 - nn' = 1 mod r =>
        // nn' = -1 mod r =>
        // n(-n^-1) = 1 mod r =>
        // n' = -n^-1 mod r =>
        // n' = (r - n)^-1 mod r
        let n_prime = mod_inverse(r - n, r).unwrap() % r;





        MontgomeryForm {
            exp,
            r,
            r_squared: mod_mult(r, r, n),
            r_inv,
            n,
            n_prime
        }
    }

    fn enter(&self, val: u128) -> MontgomeryElt {
        let new_val = self.redc(mod_mult(val, self.r_squared, self.n));

        MontgomeryElt::new(
            Box::new(self.clone()),
            new_val
        )
    }


    fn redc(&self, val: u128) -> u128 {
        let little_m = mod_mult(val, self.n_prime, self.r);
        let new_t = (val + (little_m * self.n)) >> self.exp;

        if new_t >= self.n {
            new_t - self.n
        } else {
            new_t
        }
    }
}

impl MontgomeryElt {
    fn new(space: Box<MontgomeryForm>, val: u128) -> Self {
        MontgomeryElt {
            space,
            val
        }
    }

    fn exit(&self) -> u128 {
        self.space.redc(self.val) % self.space.n
    }

    fn mult(&self, other: &MontgomeryElt) -> MontgomeryElt {
        MontgomeryElt {
            space: self.space.clone(),
            val: self.space.redc(self.val.wrapping_mul(other.val))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wikipedia_example() {
        let space = MontgomeryForm::new(17, 8);
        let a = space.enter(7);
        let b = space.enter(15);
        let c = a.mult(&b);
        assert_eq!(c.exit(), (7 * 15) % 17);
    }

    #[quickcheck]
    fn multiplication(a_in: u128, b_in: u128, n: u128) -> bool {
        if n <= 1 { return true; }
        if gcd(n, 1 << 8) != 1 { return true; }

        let space = MontgomeryForm::new(n, 8);
        let a = space.enter(a_in);
        let b = space.enter(b_in);
        let c = a.mult(&b);
        c.exit() == mod_mult(a_in, b_in, n)
    }

    #[test]
    fn edge() {
        let a_in = 1; let b_in = 1; let n = 9; let r_exp = 8;

        let space = MontgomeryForm::new(n, r_exp);
        let a = space.enter(a_in);
        let b = space.enter(b_in);
        let c = a.mult(&b);

        dbg!(&a,&b,&c);
        dbg!(mod_mult(a_in, b_in, n));

        assert_eq!(c.exit(), mod_mult(a_in, b_in, n));
    }
}
