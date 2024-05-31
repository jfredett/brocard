#![feature(const_swap, const_mut_refs)]
#![feature(duration_abs_diff)]

pub mod montgomery;
pub mod math;
pub mod brocard;

// Test Deps
#[cfg(test)]
#[macro_use] extern crate quickcheck_macros;

extern crate crossbeam;
