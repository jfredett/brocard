use crate::brocard::candidate::BrocardCandidate;
use std::time::{Duration, Instant};
use std::io::Write;

#[derive(Debug, PartialEq, Clone)]
pub struct BrocardReport {
    pub candidates: Vec<BrocardCandidate>,
    pub primes: Vec<u128>,
    pub start_time: Instant,
    pub duration: Duration,
}

impl BrocardReport {
    pub fn new(primes: Vec<u128>) -> Self {
        BrocardReport {
            candidates: vec![],
            primes,
            start_time: Instant::now(),
            duration: Duration::new(0, 0), // a placeholder
        }
    }

    pub fn push(&mut self, candidate: BrocardCandidate) {
        self.candidates.push(candidate);
    }

    pub fn finish(&mut self) -> &mut Self {
        self.duration = std::time::Instant::now().duration_since(self.start_time);
        self
    }

    pub fn write_to_file(&self, filename: &str) -> Result<(), std::io::Error> {
        let mut file = std::fs::File::create(filename).unwrap();
        let mut max = 0;
        let mut max_passed = 0;


        for candidate in &self.candidates {
            match candidate {
                BrocardCandidate::Solution(n) => {
                    writeln!(file, "S:{}", n).unwrap();
                }
                BrocardCandidate::Nonsolution { candidate, passed } => {
                    if *passed > max_passed {
                        max = *candidate;
                        max_passed = *passed;
                    }
                }
            }
        }
        writeln!(file)?;

        writeln!(file, "N:{},M:{},{}", self.candidates.len(), max, max_passed)?; 
        write!(file, "P:")?;
        for p in &self.primes {
            write!(file, "{},", p)?;
        }
        writeln!(file)
    }
}
