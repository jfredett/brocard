use crate::montgomery::Space;
use crate::math::legendre::LegendreSymbol;
use std::ops::{Add, Sub, Mul, MulAssign, AddAssign};

#[derive(Debug, Clone, Copy)]
pub struct Elt<'a, const R_EXP: usize> {
    pub val: u128,
    pub space: &'a Space<R_EXP>
}

impl<const R_EXP: usize> Elt<'_, R_EXP> {
    #[inline] pub fn exit(&self) -> u128 {
        self.space.redc(self.val) % self.space.n
    }

    #[inline] pub fn exp(&self, e: u128) -> Elt<R_EXP> {
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

    #[inline] pub fn legendre(&self) -> LegendreSymbol {
        self.space.legendre(*self)
    }
}

impl<const R_EXP: usize> PartialEq for Elt<'_, R_EXP> {
    #[inline] fn eq(&self, other: &Elt<R_EXP>) -> bool {
        self.val == other.val && self.space == other.space
    }
}

impl<const R_EXP: usize> AddAssign for Elt<'_, R_EXP> {
    #[inline] fn add_assign(&mut self, other: Elt<R_EXP>) {
        self.val = (self.val + other.val) % self.space.n;
    }
}

impl<const R_EXP: usize> MulAssign for Elt<'_, R_EXP> {
    #[inline] fn mul_assign(&mut self, other: Elt<R_EXP>) {
        self.val = self.space.redc(self.val.wrapping_mul(other.val));
    }
}
impl<const R_EXP: usize> MulAssign<u128> for Elt<'_, R_EXP> {
    #[inline] fn mul_assign(&mut self, other: u128) {
        *self *= self.space.enter(other);
    }
}

impl<'a, const R_EXP: usize> Add<Elt<'a, R_EXP>> for Elt<'a, R_EXP> {
    type Output = Elt<'a, R_EXP>;

    #[inline] fn add(self, other: Elt<'a, R_EXP>) -> Elt<'a, R_EXP> {
        Elt {
            val: self.val.wrapping_add(other.val),
            space: self.space
        }
    }
}

impl<'a, const R_EXP: usize> Add<u128> for Elt<'a, R_EXP> {
    type Output = Elt<'a, R_EXP>;

    #[inline] fn add(self, other: u128) -> Elt<'a, R_EXP> {
        self + self.space.enter(other)
    }

}

impl<'a, const R_EXP: usize> Mul for Elt<'a, R_EXP> {
    type Output = Elt<'a, R_EXP>;

    #[inline] fn mul(self, other: Elt<'a, R_EXP>) -> Elt<'a, R_EXP> {
        Elt {
            val: self.space.redc(self.val.wrapping_mul(other.val)),
            space: self.space
        }
    }
}

impl<'a, const R_EXP: usize> Mul<u128> for Elt<'a, R_EXP> {
    type Output = Elt<'a, R_EXP>;

    #[inline] fn mul(self, other: u128) -> Elt<'a, R_EXP> {
        self * self.space.enter(other)
    }
}

impl<'a, const R_EXP: usize> Sub for Elt<'a, R_EXP> {
    type Output = Elt<'a, R_EXP>;

    #[inline] fn sub(self, other: Elt<'a, R_EXP>) -> Elt<'a, R_EXP> {
        Elt {
            val: (self.val + (self.space.n - other.val)) % self.space.n,
            space: self.space
        }
    }
}
