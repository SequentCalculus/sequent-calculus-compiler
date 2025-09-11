//! This module defines some utilities for the command-line options.

/// This enum encodes for which backend to perform a command.
#[derive(clap::ValueEnum, Clone)]
pub enum Backend {
    Aarch64,
    Rv64,
    X86_64,
}
