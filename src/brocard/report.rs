use crate::brocard::candidate::BrocardCandidate;
use std::time::{Duration, Instant};
use std::io::Write;

#[derive(Debug, PartialEq, Clone)]
pub struct BrocardReport {
    pub candidates: Vec<BrocardCandidate>,
    pub start_time: Instant,
    pub duration: Duration,
}

impl BrocardReport {
    pub fn empty() -> Self {
        BrocardReport {
            candidates: vec![],
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

    pub fn write_to_file(&self, filename: &str) {
        let mut file = std::fs::File::create(filename).unwrap();
        for candidate in &self.candidates {
            match candidate {
                BrocardCandidate::Solution(n) => {
                    writeln!(file, "S:{}", n).unwrap();
                }
                BrocardCandidate::Nonsolution { candidate, passed } => {
                    writeln!(file, "N:{},{}", candidate, passed).unwrap();
                }
            }
        }
    }
}
