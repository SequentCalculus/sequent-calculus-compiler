//! This module defines the syntax for the surface language Fun.

/// Type Alias for Variables.
pub type Var = String;
/// Type Alias for Covariables.
pub type Covar = String;
/// Type alias for names of top-level functions, user-declared types and xtors
pub type Name = String;

pub mod context;
pub mod declarations;
pub mod substitution;
pub mod terms;
pub mod types;
