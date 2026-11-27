//! This module defines typechecking in Fun. Signatures of top-level functions are annotated, all
//! other types are inferred, including the overload of each call (see [`inference`]). Moreover,
//! we monomorphize all user-declared types by instantiating their type parameters whenever we
//! encounter an instance we have not seen yet.

pub mod errors;
pub mod inference;
pub mod symbol_table;

pub use errors::Error;
pub use symbol_table::{SymbolTable, build_symbol_table};
