//! This crate is an abstraction layer for the compilation of [AxCut](axcut) programs to machine
//! code. It abstracts much of the logic for code generation via several traits which concrete
//! backend platforms can implement. The traits are:
//!
//! - [`code::Instructions`]
//! - [`config::Config`]
//! - [`memory::Memory`]
//! - [`parallel_moves::ParallelMoves`]
//! - [`utils::Utils`]
//!
//! These traits have several type parameters, which must be instantiated by the platforms:
//!
//! - `Code` stands for the concrete instruction set.
//! - `Temporary` stands for the concrete implementation of temporaries, which typically includes
//!   registers and possibly spill spots.
//! - `Immediate` stands for immediate integers (this will likely change when more built-in types
//!   are supported).
//!
//! Moreover, several functions in this crate abstract a type parameter `Backend` which is used to
//! distinguish the platforms and can be implemented by the latter as an empty struct.
//!
//! Finally, trait [`statements::CodeStatement`] provides the code generation logic via method
//! [`statements::CodeStatement::code_statement`] implemented by each [AxCut](axcut) syntax node.
//!
//! # Usage
//!
//! Function [`coder::compile`] compiles an [AxCut](axcut) program to a [`coder::AssemblyProg`]
//! which contains the instructions and the number of arguments. The necessary plumbing to turn
//! these instructions into a complete assembly routine must be implemented by each backend
//! platform.

pub mod code;
pub mod coder;
pub mod config;
pub mod fresh_labels;
pub mod memory;
pub mod parallel_moves;
pub mod statements;
pub mod substitution;
pub mod utils;
