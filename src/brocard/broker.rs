use crate::math::gcd;
use crate::math::prime::*;
use crossbeam::channel::{Receiver, Sender};
use std::time::Duration;

use crate::brocard::span::BrocardSpan;
use crate::brocard::report::BrocardReport;
use crate::brocard::candidate::BrocardCandidate;


pub struct BrocardBroker {
    start: u128,
    span: u128,
    chunk_size: usize,
    target_time: std::time::Duration,
    rx: Receiver<BrocardReport>,
    tx: Sender<BrocardReport>
}


impl BrocardBroker {
    pub fn new(start: u128, span: u128, chunk_size: usize, target_time: Duration) -> BrocardBroker {
        let (tx, rx) = crossbeam::channel::unbounded();

        if chunk_size as u128 > span { panic!("Chunk Size must be less than the Span"); }
        if target_time.as_secs() == 0 && target_time.subsec_nanos() == 0 { panic!("Target time must be greater than 0"); }

        BrocardBroker {
            start,
            span,
            chunk_size,
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
        let mut backoff = workload_max;

        loop {
            // if the pool is saturated (all threads are busy), then we need to wait for a job to
            // finish
            if active_jobs >= workload_max {
                // TODO: The receiver should be on it's own thread, outside the pool. The broker
                // just starts and manages those two threads.
                loop {
                    // check the talkback channel for a message that a job has finished
                    // TODO: Would be better to spin this till the queue is empty before spawning
                    // new jobs?
                    // FIXME: start_time is kinda busted, I want this to be the actual start time,
                    // but Instant doesn't work that way
                    if let Ok(ref report@BrocardReport { ref candidates, start_time: _, primes: _, duration  } ) = self.rx.try_recv() {
                        // TODO: impl Display for stuff instead of picking it apart here.
                        println!("Received Report from chunk started {:?} ago.", duration);

                        print!("Writing report to compressed file... ");
                        // TODO: This feels kind of crappy.
                        // TODO: This should also compress the file.
                        let write_result = report.write_to_file(
                            format!("./out/report-{}.json", started_jobs - workload_max).as_str()
                        );

                        match write_result {
                            Ok(_) => println!("Done."),
                            Err(e) => println!("Failed to write chunk due to: {:?}", e)
                        }

                        let mut max = 0;
                        let mut max_passed = 0;
                        let mut sols = 0;
                        for candidate in candidates {
                            match candidate {
                                BrocardCandidate::Solution(n) => {
                                    sols += 1;
                                    println!("Found Solution: {}", n);
                                }
                                BrocardCandidate::Nonsolution { candidate, passed } => {
                                    if *passed > max_passed {
                                        max = *candidate;
                                        max_passed = *passed;
                                    }
                                }
                            }
                        }

                        println!("Found {} Solutions", sols);

                        println!("Found {} Nonsolutions", candidates.len() - sols);
                        println!("Nonsolution that passed the most tests: {} with {}/{}", max, max_passed, prime_count);

                        total_checked += candidates.len() as u128;

                        println!("Adjusting Size to match target time.");
                        let delta = duration.abs_diff(self.target_time);
                        println!("Current Delta: {:?}", delta);
                        // If we're more than 2% off the target time
                        if delta > (self.target_time / 50) && backoff == 0 {
                            // This will ensure we only update after a reasonable number of jobs
                            // come in.
                            backoff = workload_max;
                            // Then we want to adjust the size of the chunk.
                            //
                            // This uses a 2,3-search. Each iteration, it'll either halve the
                            // size, or advance it by a 3x of it's current size. Over enough
                            // iterations, it should converge to a size of chunk that will be
                            // sufficiently close to the target that we get into about 2% of the
                            // target time. This is heuristic, but the goal is to avoid losing more
                            // than some target amount of work in case something fails.
                            //
                            // It's slower than a binary search, but I don't have to keep track of
                            // the previous iteration, and it should be good enough to tune the
                            // thing.
                            if duration > self.target_time {
                                self.chunk_size = self.chunk_size / 2;
                            } else {
                                self.chunk_size *= 3;
                            }
                            // TODO: Proper logging framework.
                            println!("Adjusted chunk size to: {} to attempt to cancel out {:?} of difference.", self.chunk_size, delta);
                        } else {
                            backoff -= 1;
                            println!("Backoff: {}", backoff);
                        }

                        println!("Chunk size is currently: {}.", self.chunk_size);
                        println!("Remaining Chunks: {}", (self.span - total_checked) / self.chunk_size as u128);
                        println!("Total Chunks: {}%", (total_checked as f64 / self.span as f64) * 100.0);
                        println!();

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

            println!("Preparing Chunk #{}", started_jobs);
            let next_start = self.start + (started_jobs * self.chunk_size) as u128;
            println!("Next chunk will start at {}", next_start);

            active_jobs += 1;
            started_jobs += 1;


            println!("Finding Primes");
            // FIXME: remove magic number (shoudl be R_EXP)
            let primes : Vec<u128> = primes_from(next_start + self.chunk_size as u128 + 1)
                                     .filter(|&n| gcd(n, 64) == 1)
                                     .take(prime_count).collect();

            let tx_0 = self.tx.clone();
            let span = BrocardSpan::new(next_start, self.chunk_size as u128, primes, tx_0);

            if next_start > self.start + self.span {
                println!("Finished all chunks.");
                break;
            }

            println!("Starting Solve for Chunk #{}", started_jobs);
            pool.spawn(move || span.solve());
        }
    }
}
