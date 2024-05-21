use crate::montgomery::Space;
use std::ops::{Add, Sub, Mul, MulAssign, AddAssign};

#[derive(Debug, Clone, Copy)]
pub struct Elt<'a> {
    pub val: u128,
    pub space: &'a Space
}

impl Elt<'_> {
    pub fn exit(&self) -> u128 {
        self.space.redc(self.val) % self.space.n
    }

    pub fn exp(&self, e: u128) -> Elt {
        let mut val = self.space.enter(1);
        let mut base = self.clone();
        let mut exp = e;

        while exp > 0 {
            if exp & 1 == 1 { val = val * base; }
            base = base * base;
            exp >>= 1;
        }

        val
    }
}

impl AddAssign for Elt<'_> {
    fn add_assign(&mut self, other: Elt) {
        self.val = (self.val + other.val) % self.space.n;
    }
}

impl MulAssign for Elt<'_> {
    fn mul_assign(&mut self, other: Elt) {
        self.val = self.space.redc(self.val.wrapping_mul(other.val));
    }
}

impl<'a> Add for Elt<'a> {
    type Output = Elt<'a>;

    fn add(self, other: Elt<'a>) -> Elt<'a> {
        Elt {
            val: self.val.wrapping_add(other.val) % self.space.n,
            space: self.space
        }
    }
}

impl<'a> Mul for Elt<'a> {
    type Output = Elt<'a>;

    fn mul(self, other: Elt<'a>) -> Elt<'a> {
        Elt {
            val: self.space.redc(self.val.wrapping_mul(other.val)),
            space: self.space
        }
    }
}

impl<'a> Sub for Elt<'a> {
    type Output = Elt<'a>;

    fn sub(self, other: Elt<'a>) -> Elt<'a> {
        Elt {
            val: (self.val + (self.space.n - other.val)) % self.space.n,
            space: self.space
        }
    }
}


