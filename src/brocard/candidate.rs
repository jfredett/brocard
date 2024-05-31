
/// This is produced by the BrocardSpan::solve method, and is used to report back to the parent
/// process the results of the computation for each item. The parent can then log these to whatever 
/// log source is convenient (probably stdout)
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BrocardCandidate {
    Nonsolution {
        candidate: u128,
        passed: usize
    },
    Solution(u128)
}

impl BrocardCandidate {
    #[inline]
    pub fn is_solution(&self) -> bool {
        match self {
            BrocardCandidate::Solution(_) => true,
            _ => false
        }
    }

    #[inline(always)]
    pub fn is_nonsolution(&self) -> bool {
        !self.is_solution()
    }

    pub fn passed(&self) -> Option<usize> {
        match self {
            BrocardCandidate::Nonsolution { passed, .. } => Some(*passed),
            _ => None
        }
    }
}
