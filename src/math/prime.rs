use crate::math::mod_exp;

// IDEA: Implement a MR Prime Basis finder using GA.


//
const MR_BASES: [u128; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
//
//

// Miller-Rabin Primality test using MR_BASES as it's set of bases. Implemented following:
// https://cp-algorithms.com/algebra/primality_tests.html
pub fn is_prime(n: u128) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n & 1 == 0 { return false; }

    let mut d = n - 1;
    let mut r = 0;
    while d & 1 == 0 {
        d >>= 1;
        r += 1;
    }

    for &a in MR_BASES.iter() {
        if a == n { return true; }
        if check_composite(n, a, d, r) { return false; }
    }

    return true;
}

fn check_composite(n: u128, a: u128, d: u128, s: u128) -> bool {
    let mut x = mod_exp(a,d,n);

    if x == 1 || x == n - 1 { return false; }

    for _ in 1..s {
        x = mod_exp(x, 2, n);
        if x == n - 1 { return false; }
    }

    return true;
}

pub fn segmented_seive(low: u128, high: u128) -> Vec<u128> {
    let mut primes = vec![];
    let mut is_prime = vec![true; (high - low + 1) as usize];

    for p in 2.. {
        if p * p > high { break; }

        let mut start = (low + p - 1) / p * p;
        if start < p * p { start = p * p; }

        for i in (start..=high).step_by(p as usize) {
            is_prime[i as usize - low as usize] = false;
        }
    }

    for i in 0..is_prime.len() {
        if is_prime[i] {
            primes.push(i as u128 + low);
        }
    }

    return primes;
}


#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_PRIMES : [u128; 168] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
        283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397,
        401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503,
        509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619,
        631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743,
        751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863,
        877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997
    ];

    fn naive_is_prime(a: u128) -> bool {
        if a < 2 { return false; }
        for p in SMALL_PRIMES.iter() {
            if a == *p { return true; }
            if a % p == 0 { return false; }
        }
        return true;
    }

    #[quickcheck]
    fn miller_rabin_correctness(a_in: u16) -> bool {
        let a = a_in as u128; // keep the size small by restricting to u16, but we need to work on
                              // u128s
        naive_is_prime(a) == is_prime(a)
    }

    #[test]
    fn segmented_seive_finds_small_primes() {
        let primes = segmented_seive(0, 1000);
        for p in SMALL_PRIMES.iter() {
            assert!(primes.contains(p));
        }
    }

    #[test]
    fn miller_rabin_small_primes() {
        for p in SMALL_PRIMES.iter() {
            assert!(is_prime(*p));
        }
    }
}

pub fn take_primes(start: u128, count: usize) -> Vec<u128> {
    todo!()
}
