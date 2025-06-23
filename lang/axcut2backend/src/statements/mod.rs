//! This module contains the implementation for the code generation logic of each AxCut construct.
//! It provides a trait [`statement::CodeStatement`] with a method
//! [`statement::CodeStatement::code_statment`] implemented by each AxCut syntax node.

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
