#![feature(allocator_api)]
use std::alloc::System;

pub mod parser;
pub mod matrix;
pub mod tree;

#[global_allocator]
static A: System = System;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
