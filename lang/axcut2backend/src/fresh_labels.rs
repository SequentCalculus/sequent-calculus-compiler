//! This module provides a source of fresh labels during code generation.

static mut COUNTER: usize = 0;

/// This function is used to generate fresh labels by incrementing and returning a global counter.
pub fn fresh_label() -> usize {
    unsafe {
        COUNTER += 1;
        COUNTER
    }
}
