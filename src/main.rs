use brocard::brocard::*;
use brocard::math::prime::*;
use std::time::Duration;

const BOUND : u128 = 1_000_000_000;

fn main() {
    BrocardBroker::new(
        2, 
        BOUND, 
        100_000_000, 
        Duration::from_secs(60)
    ).run_solver(60);
}
