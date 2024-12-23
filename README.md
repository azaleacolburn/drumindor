# drumindor

My personal no_std implementations of everything I can think of in Rust.

### Exceptions

- After defining a custom allocator, other structs use the system allocator. We're doing this not because our allocator isn't good enough, but because setting a global allocator in a library is painful, plus we'd have to use `extern crate alloc` anyways to call our global allocator.
