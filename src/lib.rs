#![no_std]

// #[global_allocator]
// static ALLOCATOR: crate::alloc::Allocator = crate::alloc::Allocator::new();

// This is fine because we've already defined a custom allocator that we can rigorously test
extern crate alloc as alloc_callable;
#[allow(unused_imports)]
use alloc_callable::alloc::{alloc, alloc_zeroed, dealloc, realloc};

pub mod alloc;
pub mod vector;
