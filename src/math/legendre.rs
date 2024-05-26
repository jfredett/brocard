use std::ops::Mul;
use crate::math::mod_exp;


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LegendreSymbol {
    Divisor = 0,
    Residue = 1,
    Nonresidue = 2,
}

impl LegendreSymbol {
    pub fn naive_legendre(a: u128, p: u128) -> LegendreSymbol {
        let ret = mod_exp(a, (p - 1) / 2, p);
        // the following is a switch statement on the output of the mod_exp above
        if ret == 0 {
            LegendreSymbol::Divisor
        } else if ret == 1 {
            LegendreSymbol::Residue
        } else {
            LegendreSymbol::Nonresidue
        }
    }
}

impl Mul for LegendreSymbol {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        match (self, rhs) {
            (LegendreSymbol::Divisor    , _                         ) => LegendreSymbol::Divisor,
            (_                          , LegendreSymbol::Divisor   ) => LegendreSymbol::Divisor,
            (LegendreSymbol::Residue    , LegendreSymbol::Residue   ) => LegendreSymbol::Residue,
            (LegendreSymbol::Residue    , LegendreSymbol::Nonresidue) => LegendreSymbol::Nonresidue,
            (LegendreSymbol::Nonresidue , LegendreSymbol::Residue   ) => LegendreSymbol::Nonresidue,
            (LegendreSymbol::Nonresidue , LegendreSymbol::Nonresidue) => LegendreSymbol::Residue,
        }
    }
}


#[cfg(test)]
mod tests {

}
