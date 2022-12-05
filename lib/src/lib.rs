#![feature(allocator_api)]
use std::alloc::{System};

pub mod parser;
pub mod matrix;

#[global_allocator]
static A: System = System;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
