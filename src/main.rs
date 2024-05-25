use brocard::montgomery::*;
use brocard::brocard::*;
use brocard::prime::*;

fn main() {
    let primes_vec = segmented_seive(1_000_000, 1_200_000);
    let mut primes = [0; 60];

    for i in 0..60 {
        primes[i] = primes_vec[i];
    }

    let span = BrocardSpan::new(2, 1_000_000, primes);
    let (potentials, exclusions) = span.solve();
    println!("found {:?} as potential solutions", potentials);
    println!("found {} total witnesses of nonsolution", exclusions.len());
}
