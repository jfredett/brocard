use crate::montgomery::*;
use rayon::prelude::*;


/// Tests all the values within [start,start+span] against the given prime and reports back any
/// successes
///
/// FIXME: this will mean that even if I find a witness to non-solution, I will still have to
/// test against every prime in my list, that's not ideal. This should take an array of primes
/// instead, and then return only those that fail for every prime in the range.
pub struct BrocardSpan {
    start: u128,
    span: u128,
    primes: [u128; 60],
}

/// This is produced by the BrocardSpan::solve method, and is used to report back to the parent
/// process the results of the computation for each item. The parent can then log these to whatever 
/// log source is convenient (probably stdout)
enum BrocardReport {
    Nonsolution(u128, usize),
    Solution(u128)
}

const R_EXP : usize = 64;

impl BrocardSpan {
    // TODO: have it automatically calculate the primes it needs? Or maybe wrap this in another
    // object which creates and manages spans and does that? not sure.
    pub fn new(start: u128, span: u128, primes: [u128; 60]) -> BrocardSpan {
        BrocardSpan {
            start,
            span,
            primes,
        }
    }

    pub fn solve(&self) -> (Vec<u128>, Vec<u128>) {
        // 1. line up all the primes and build montgomery spaces around them
        let spaces : Vec<Space> = self.primes.iter().map(|p| Space::new(*p, R_EXP)).collect();
        // 2. identify the 'candidate', which is START
        let mut candidate = self.start;
        // 3. create an initial vector of elts V_i = p_i.factorial(candidate) that all represent
        //    the factorial of the current candidate in each montgomery space
        let mut v : Vec<Elt> = spaces.par_iter().map(|s| s.factorial(candidate)).collect();

        // TODO: Replace w/ a single vector of BrocardReports
        let mut potentials = vec![];
        let mut exclusions = vec![];

        loop {
            // 4.1. calculate the value of the legendre symbol `V_i R p_i` in parallel, halting
            //      all threads when a non-solution-witness is found.
            // TODO: 1. Verify this short circuits.
            // TODO: 2. It would be nice to get the count of how many passed, but not critical
            // TODO: 3. This is a little ugly, maybe wrapping up the Primes in it's own object
            // would make it nicer?
            let test : bool = v.par_iter().all(|v_i| {
                            (*v_i + 1).legendre() != LegendreSymbol::Nonresidue
                        });

            if test {
                // 4.2.1 if all the legendre symbols are residues, add the candidate to the list of
                //       potential solutions
                potentials.push(candidate);
            } else {
                // 4.2.2 if any of the legendre symbols are non-residues, add the candidate to the
                //       list of non-solutions
                exclusions.push(candidate);
            }

            // 4.3. increment the candidate by one, 
            candidate += 1;
            // 4.4. if candidate > START + SPAN, break
            if candidate > self.start + self.span {
                break;
            }
            // 4.5. multiply `V_i * p.enter(candidate)` for all i. This set's V_i = (i+1)!
            v.par_iter_mut().for_each(|v_i| {
                *v_i *= candidate
            });
        }
        // 5. return the list of candidates that passed the test. Additionally return metadata
        //    about time spent, etc, for optimization
        return (potentials, exclusions);



        // The intent is that these spans are broken from a larger range, and then the results are
        // aggregated by the parent and more work is assigned. The parent can use the metadata to
        // further optimize what work is to be done.
        //
        // The parent will execute many brocardspans in parallel, and brocardspans themselves can
        // parallelize code, in particular steps 1,3, 4.1, and 4.5 all seem natural to parallelize,
        // and the loop itself can be streamlined (and in fact likely SIMDified).
    }
}

/*
 *
 *
    // step 1: generate primes 
    //
    let primes = segmented_seive(UPPER_BOUND as u128, (UPPER_BOUND + 100_000) as u128)
               .into_iter().take(60)
               .map(|p| Space::new(p, R_EXP))
               .collect::<Vec<_>>()
                                                                                                                 ;
    (START..UPPER_BOUND).into_par_iter().for_each( |candidate| {
        let mut solution = true;

        for p_space in &primes {
            let a = p_space.factorial(candidate) + p_space.enter(1);
            match p_space.legendre(a) {
                LegendreSymbol::Nonresidue => {
                    solution = false;
                    break;
                },
                _ => ()
            }
        }

        if solution {
            println!("{} is a solution", candidate);
        } else {
            if candidate % 1000 == 0 {
                println!("{}/{}", candidate, UPPER_BOUND);
            }
        }
    })
    */
