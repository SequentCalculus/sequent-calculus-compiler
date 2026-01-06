//! This module defines typechecking in Fun. We infer some of the types but rely on type
//! annotations for signatures of top-level functions and let-bindings. Moreover, we monomorphize
//! all user-declared types by instantiating their type parameters whenever we encounter an
//! instance we have not seen yet. All types annotated in terms must be monomorphic. For
//! monomorphization we rely on annotations of type arguments in pattern matches and destructor
//! invocations.

pub mod check;
#[allow(unused_assignments)]
pub mod errors;
pub mod symbol_table;

pub use check::{Check, check_args, check_equality};
pub use errors::Error;
pub use symbol_table::{SymbolTable, build_symbol_table};
