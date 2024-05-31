use crate::montgomery::*;
use crate::math::legendre::*;
use crate::brocard::report::*;
use crate::brocard::candidate::*;
use crossbeam::channel::Sender;

/// Tests all the values within [start,start+span] against the given prime and reports back any
/// successes
///
/// FIXME: this will mean that even if I find a witness to non-solution, I will still have to
/// test against every prime in my list, that's not ideal. This should take an array of primes
/// instead, and then return only those that fail for every prime in the range.
///
pub struct BrocardSpan {
    start: u128,
    span: u128,
    primes: Vec<u128>,
    tx: Sender<BrocardReport>, // probably need to put a lock around this?
}

const R_EXP : usize = 64;

impl BrocardSpan {
    // TODO: have it automatically calculate the primes it needs? Or maybe wrap this in another
    // object which creates and manages spans and does that? not sure.
    pub fn new(start: u128, span: u128, primes: Vec<u128>, tx: Sender<BrocardReport>) -> BrocardSpan {
        BrocardSpan {
            start,
            span,
            primes,
            tx
        }
    }

    pub fn solve(&self) {
        let mut result = BrocardReport::empty();

        // 1. line up all the primes and build montgomery spaces around them
        let spaces : Vec<Space<R_EXP>> = self.primes.iter().map(|p| Space::new(*p)).collect();
        // 2. identify our first candidate to try
        let mut candidate = self.start;
        // 3. create an initial vector of elts V_i = p_i.factorial(candidate) that all represent
        //    the factorial of the current candidate in each montgomery space 
        let mut v : Vec<Elt<R_EXP>> = spaces.iter().map(|s| s.factorial(candidate)).collect();


        loop {
            // 4.1. calculate the value of the legendre symbol `V_i R p_i`, halting all threads
            //      when a non-solution-witness is found.
            // TODO: 2. It would be nice to get the count of how many passed, but not critical
            // TODO: 3. This is a little ugly, maybe wrapping up the Primes in it's own object
            // would make it nicer?
            let test : Vec<_> = v.iter().filter(|v_i| {
                (**v_i + 1).legendre() != LegendreSymbol::Nonresidue
            }).collect();

            if test.is_empty() {
                // 4.2.1 if any of the legendre symbols are non-residues, add the candidate to the
                //       list of non-solutions
                result.push(BrocardCandidate::Solution(candidate));
            } else {
                // 4.2.2 if all the legendre symbols are residues, add the candidate to the list of
                //       potential solutions
                result.push(BrocardCandidate::Nonsolution { candidate, passed: test.len() });
            }

            // 4.3. increment the candidate by one, 
            candidate += 1;

            // 4.4. if candidate > START + SPAN, break
            if candidate > self.start + self.span {
                break;
            }

            // 4.5. multiply `V_i * p.enter(candidate)` for all i. This set's V_i = (i+1)!
            v.iter_mut().for_each(|v_i| {
                *v_i *= candidate
            });
        }
        // 5. return the list of candidates that passed the test. Additionally return metadata
        //    about time spent, etc, for optimization
        let ret = result.finish();
        self.tx.send(ret.clone());
    }

}
