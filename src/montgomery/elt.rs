use crate::montgomery::Space;
use std::ops::{Add, Sub, Mul};

pub struct Elt<'a> {
    pub val: u128,
    pub space: &'a Space
}

impl Elt<'_> {
    pub fn exit(&self) -> u128 {
        self.space.redc(self.val) % self.space.n
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


