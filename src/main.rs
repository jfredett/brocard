#![feature(portable_simd)]

pub mod mult;

// Test Deps
#[cfg(test)]
#[macro_use] extern crate quickcheck_macros;

fn main() {
    println!("Hello, world!");
}
