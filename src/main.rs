use brocard::brocard::*;
use brocard::math::prime::*;

const BOUND : u128 = 100_000_000;

fn main() {
    let pool = rayon::ThreadPoolBuilder::new().build().unwrap();
    println!("{:?}", pool.current_num_threads());
    /*
    println!("Calculating Primes");
    let primes_vec = segmented_seive(BOUND, BOUND + 50_000);
    let mut primes = [0; 60];


    for i in 0..60 {
        primes[i] = primes_vec[i];
    }

    println!("Starting Solve");

    let span = BrocardSpan::new(2, BOUND, primes);
    let (potentials, exclusions) = span.solve();
    println!("found {:?} as potential solutions", potentials);
    println!("found {} total witnesses of nonsolution", exclusions.len());
    */
}
