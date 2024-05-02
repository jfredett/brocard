fn monty_mult(a : i128, b : i128, m : i128) -> i128 {
    let r_exp = 8;
    let r = 1 << r_exp;
    // the modular inverse of m mod r
    let m_inv =  mod_inverse(m, r);
    
    // montgomery form for a and b
    // a' = a * r mod m
    let a_prime = modular_mult(a, r, m);
    let b_prime = modular_mult(b, r, m);

    // Multiply in Montgomery Form
    let mut t = a_prime * b_prime;
    let little_m = modular_mult(t, m_inv, r);
    t = (t + little_m * m) >> r_exp;

    if t >= m {
        return t - m;
    } else {
        return t;
    };
}

fn modular_exponent(mut n:i128 ,mut x:i128 , p:i128) -> i128 {
    let mut ans = 1;
    if x <= 0 { return 1; }
    loop {
        if x == 1 { return (ans * n) % p; }

        if x & 1 == 0 { 
            n = ( n * n ) % p;
            x >>= 1;
            continue; 
        } else { 
            ans = (ans*n) % p;
            x -= 1; 
        }
    }
}
 
fn modular_mult(mut n:i128 ,mut x:i128 , p:i128) -> i128 {
    let mut ans = 0;
    if x <= 0 { return 1; }
    loop {
        if x == 1 { return (ans + n) % p; }

        if x & 1 == 0 { 
            n = ( n + n ) % p;
            x >>= 1;
            continue; 
        } else { 
            ans = (ans + n) % p;
            x -= 1; 
        }
    }
}

fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = extended_gcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inverse(m: i128, r: i128) -> i128 {
    let (g, x, _) = extended_gcd(m, r);
    if g != 1 {
        panic!("Inverse does not exist!");
    } else {
        // Normalize the result to be positive
        (x % r + r) % r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modular_mult() {
        assert_eq!(modular_mult(2, 4, 5), 3);
    } 

    #[test]
    fn test_exponent() {
        assert_eq!(modular_exponent(2, 4, 5), 1);
    }

    #[quickcheck]
    fn modular_mult_is_naive_mult(n : i128, x : i128, p : i128) -> bool {
        if x <= 0 { return true; }
        if p <= 1 { return true; }
        modular_mult(n, x, p) == (n * x) % p
    }

    #[quickcheck]
    fn modular_exp_is_naive_exp(n : i128, x : i128, p : i128) -> bool {
        // These constraints are necessary so the naive side doesn't overflow.
        // the exponent must also be > 0
        if x <= 0 { return true; }
        if x >= 16 { return true; }

        if p <= 1 { return true; }

        modular_exponent(n, x, p) == (n.pow(x as u32)) % p
    }

    #[test]
    fn modular_inverse_works() {
        assert_eq!(mod_inverse(3, 5), 2);
    }

    #[quickcheck]
    fn monty_mult_is_modular_mult(a : i128, b : i128, m : i128) -> bool {
        if m <= 1 { return true; }
        if m % 2 == 0 { return true; }
        if a <= 1 { return true; }
        if b <= 1 { return true; }
        if a >= m { return true; }
        if b >= m { return true; }
        


        dbg!(mod_inverse(a,m));
        dbg!(mod_inverse(b,m));

        monty_mult(a, b, m) == modular_mult(a, b, m)
    }
}
