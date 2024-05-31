use crate::math::prime::*;
use crossbeam::channel::{Receiver, Sender};
use std::time::Duration;

use crate::brocard::span::BrocardSpan;
use crate::brocard::report::BrocardReport;


pub struct BrocardBroker {
    start: u128,
    span: u128,
    size: usize,
    target_time: std::time::Duration,
    rx: Receiver<BrocardReport>,
    tx: Sender<BrocardReport>
}


impl BrocardBroker {
    pub fn new(start: u128, span: u128, size: usize, target_time: Duration) -> BrocardBroker {
        let (tx, rx) = crossbeam::channel::unbounded();

        if span > size as u128 { panic!("Total checked span must be less than or equal to chunk size"); }
        if target_time.as_secs() == 0 && target_time.subsec_nanos() == 0 { panic!("Target time must be greater than 0"); }

        BrocardBroker {
            start,
            span,
            size,
            target_time,
            rx,
            tx
        }
    }

    pub fn run_solver(&mut self, prime_count: usize) {
        println!("Building Threadpool");

        let pool = rayon::ThreadPoolBuilder::new().build().unwrap();

        println!("Starting Solvers");

        // Dynamically sizing to ensure best performance and highest recoverability.
        // Ultimately we want to proceed in chunks on each processor available to us, 
        // to do that we need to break into chunks of a reasonable size,
 

        let workload_max = pool.current_num_threads();
        let mut active_jobs = 0;

        let mut started_jobs = 0;
        let mut total_checked = 0;

        loop {
            // if the pool is saturated (all threads are busy), then we need to wait for a job to
            // finish
            if active_jobs >= workload_max {
                loop {
                    // check the talkback channel for a message that a job has finished
                    // TODO: Would be better to spin this till the queue is empty before spawning
                    // new jobs?
                    if let Ok(ref report@BrocardReport { ref candidates, start_time, duration  } ) = self.rx.try_recv() {
                        // TODO: impl Display for stuff instead of picking it apart here.
                        println!("Received Report started at: {:?}, ({:?} ago).", start_time, duration);

                        print!("Writing report to compressed file... ");
                        // TODO: This feels kind of crappy.
                        // TODO: This should also compress the file.
                        report.write_to_file(format!("./report-{}.json", started_jobs).as_str());
                        println!("Done.");

                        let solutions = candidates.iter().filter(|c| c.is_solution());
                        let nonsolutions = candidates.iter().filter(|c| c.is_nonsolution());

                        if solutions.clone().count() > 0 {
                            // TODO: These should be ganged together and set off to a specialized
                            // Span which doublechecks the potential solutions against a larger #
                            // of primes. This will probably required BrocardSpan to take an
                            // iterator rather than a specific start/span.
                            println!("Found Solutions: {:?}", solutions.clone());
                        }

                        println!("Found {} Nonsolutions", nonsolutions.clone().count());
                        println!("Nonsolution that passed the most tests: {:?}", nonsolutions.clone().max_by_key(|c| c.passed().unwrap()).unwrap());

                        total_checked += candidates.len() as u128;


                        println!("Adjusting Size to match target time.");
                        let delta = duration - self.target_time;
                        println!("Current Delta: {:?}", delta);
                        // If we're more than 2% off the target time
                        if delta > (self.target_time / 50) {
                            // Then we want to adjust the size of the chunk.
                            //
                            // This uses a 1/2 / 4/3rds search. Each iteration, it'll either halve the
                            // size, or advance it by a 3rd of it's current size. Over enough
                            // iterations, it should converge to a size of chunk that will be
                            // sufficiently close to the target that we get into about 2% of the
                            // target time. This is heuristic, but the goal is to avoid losing more
                            // than some target amount of work in case something fails.
                            //
                            // It's slower than a binary search, but I don't have to keep track of
                            // the previous iteration, and it should be good enough to tune the
                            // thing.
                            if duration > self.target_time {
                                self.size = self.size / 2;
                            } else {
                                self.size *= 3;
                            }
                            // TODO: Proper logging framework.
                            println!("Adjusted size to: {} to attempt to cancel out {:?} of difference.", self.size, delta);
                        }

                        // TODO: If it makes an adjustment, it should also print out that it has
                        // made that adjustment, and also print out the %'age of the way through
                        // the span it is. And maybe try to guess at a time-till-completion
                        println!("Span size is currently: {}.", self.size);
                        println!("Remaining Chunks: {}", (self.span - total_checked) / self.size as u128);
                        println!("Total Chunks: {}%", (total_checked as f64 / self.span as f64) * 100.0);

                        // decrement the active jobs counter
                        active_jobs -= 1;
                        // since we have room to schedule more jobs, go do that.
                        break;
                    } 
                    // if we have room to schedule more jobs, go do that.
                    if active_jobs < workload_max { break; }
                }
            }


            // TODO: Implement take_primes, which gets the first `prime_count` primes larger than
            // the given value

            println!("Starting Chunk {}", started_jobs);
            active_jobs += 1;
            started_jobs += 1;
            let primes = take_primes(self.size as u128, prime_count);
            
            let next_start = self.start + (started_jobs * self.size as u128);

            let tx_0 = self.tx.clone();
            let span = BrocardSpan::new(next_start, self.size as u128, primes, tx_0);

            // FIXME: I think this is wrong, it's happy because I'm blocking, not because it's
            // actually going to work. I suspect this'll stay on a single thread.
            pool.install(|| span.solve());
        }
    }
}
