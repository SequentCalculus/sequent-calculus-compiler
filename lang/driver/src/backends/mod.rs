//! This module contains the compiler logic for generating assembly files and subsequent
//! compilation to object files and linking for the different backends.

pub mod aarch64;
pub mod rv64;
pub mod x86_64;
