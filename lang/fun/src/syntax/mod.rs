//! This module defines the syntax for the surface language Fun.

/// Type Alias for Variables.
pub type Var = String;
/// Type Alias for Covariables.
pub type Covar = String;
/// Type Alias for other Names (such as for top-level declarations).
pub type Name = String;

pub mod context;
pub mod declarations;
pub mod substitution;
pub mod terms;
pub mod types;
