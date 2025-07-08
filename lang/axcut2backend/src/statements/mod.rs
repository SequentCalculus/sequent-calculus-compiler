//! This module contains the implementation for the code generation logic of each [AxCut](axcut)
//! construct. It provides a trait [`CodeStatement`] with a method
//! [`CodeStatement::code_statement`] implemented by each [AxCut](axcut) syntax node.

pub mod code_statement;
pub mod create;
pub mod exit;
pub mod ifc;
pub mod ifz;
pub mod invoke;
pub mod r#let;
pub mod literal;
pub mod op;
pub mod print;
pub mod substitute;
pub mod switch;

pub use code_statement::CodeStatement;
