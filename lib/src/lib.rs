#![feature(allocator_api)]
use std::alloc::System;

pub mod parser;
pub mod matrix;
pub mod tree;

#[global_allocator]
static A: System = System;