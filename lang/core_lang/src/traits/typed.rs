//! Defines the [Typed] trait
use crate::syntax::Ty;

/// Trait for anything that has a type
pub trait Typed {
    /// Get the type of `&self`
    fn get_type(&self) -> Ty;
}
