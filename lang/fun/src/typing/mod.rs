//! This module defines typechecking in Fun. We infer some of the types but rely on type
//! annotations for signatures of top-level functions and let-bindings.

pub mod check;
pub mod errors;
pub mod symbol_table;

pub use check::{Check, check_args, check_equality};
pub use errors::Error;
pub use symbol_table::{SymbolTable, build_symbol_table};
