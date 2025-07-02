//! This crate implements the details for the x86_64 ISA. It implements the methods and type
//! parameters for the traits abstracted in [`axcut2backend`]. It is supposed to work on Linux and
//! Apple platforms (though the latter is not properly tested). For other platforms the calling
//! convention for calls to external functions and for the invocation of the generated assembly
//! routine itself need to be adapted.
//!
//! # Usage
//!
//! Function [`into_routine::into_x86_64_routine`] turns an [`axcut2backend::coder::AssemblyProg`]
//! generated from an AxCut program by [`axcut2backend::coder::compile`] into a complete assembly
//! routine which  can then be printed to a file, assembled and linked with a driver and some
//! runtime functions.

pub mod code;
pub mod config;
pub mod into_routine;
pub mod memory;
pub mod parallel_moves;
pub mod utils;

/// This empty struct implements the type parameter `Backend` which is used by the abstraction
/// layer [`axcut2backend`] to distinguish the backend platforms.
pub struct Backend;
