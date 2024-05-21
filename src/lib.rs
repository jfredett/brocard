#![feature(portable_simd)]
#![feature(const_mut_refs, const_trait_impl)]
#![feature(effects,const_for,const_swap)]

pub mod montgomery;
pub mod prime;

// Test Deps
#[cfg(test)]
#[macro_use] extern crate quickcheck_macros;
