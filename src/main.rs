#![feature(const_mut_refs)]

use rayon::prelude::*;
use brocard::prime::*;
use brocard::montgomery::*;

const START : usize = 2;
const UPPER_BOUND : usize = 1_000_000_000;
const R_EXP : usize = 64;

/* TODO: Make it parse args
#[derive(Parser, Debug)]
struct Args {
    /// Where to start searching
    #[clap(long, default_value = "2")]
    start: u128,
    /// How many values to search
    #[clap(long, default_value = "1_000_000_000")]
    span: u128
}
*/

fn main() {
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
}
