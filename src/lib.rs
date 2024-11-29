#![no_std]

pub mod alloc;
pub mod vector;

#[global_allocator]
static ALLOCATOR: alloc::Allocator = alloc::Allocator::new();
