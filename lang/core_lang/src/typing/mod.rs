//! Type checking infrastructure shared across all syntax elements: the `Checked` trait, the
//! global environment it checks against, and its error type.

pub mod check;
pub mod env;
pub mod errors;
