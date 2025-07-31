//! This module defines a trait with a method for obtaining the type of a term.

use crate::syntax::Ty;

/// This trait provides a method for obtaining the type of a term.
pub trait Typed {
    /// This method returns the type of a term.
    fn get_type(&self) -> Ty;
}
