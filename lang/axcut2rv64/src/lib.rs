//! This crate implements the details for the RISC-V rv64 ISA. It implements the methods and type
//! parameters for the traits abstracted in [`axcut2backend`]. It does currently not implement the
//! plumbing needed to turn the generated assembly code into a complete assembly routine for a
//! specific platform. This likely requires some setup and cleanup code (see the [Usage](#usage)
//! section below) and an implemention for calling runtime functions.
//!
//! # Usage
//!
//! Function [`into_routine::into_rv64_routine`] is supposed to turn an
//! [`axcut2backend::coder::AssemblyProg`] generated from an AxCut program by
//! [`axcut2backend::coder::compile`] into a complete assembly routine which can then be printed to
//! a file, assembled and linked with a driver and some runtime functions. This function thus needs
//! to be adapted for every specific hardware platform.

pub mod code;
pub mod config;
pub mod into_routine;
pub mod memory;
pub mod parallel_moves;
pub mod utils;

/// This empty struct implements the type parameter `Backend` which is used by the abstraction
/// layer [`axcut2backend`] to distinguish the backend platforms.
pub struct Backend;
