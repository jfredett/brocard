pub mod prime;
pub mod legendre;

// TODO: This almost certainly exists somewhere already
#[inline] pub const fn gcd(a: u128, b: u128) -> u128 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

#[inline] pub const fn mod_inverse(n: u128, r: u128) -> Option<u128> {
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

#[inline] pub const fn mod_mult(a: u128, b: u128, n: u128) -> u128 {
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

#[inline] pub const fn mod_inv(a: u128, n: u128) -> u128 {
    let (x, _) = extended_gcd(a, n);
    (x % n + n) % n
}

#[inline] pub const fn mod_exp(a: u128, k: u128, n: u128) -> u128 {
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

#[inline] pub const fn extended_gcd(a: u128, b: u128) -> (u128, u128) {
    let mut s = 0;
    let mut t = 1;
    let mut r = b;

    let mut old_s = 1;
    let mut old_t = 0;
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
