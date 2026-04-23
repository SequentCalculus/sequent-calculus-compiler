use crate::syntax::Ty;

/// This trait provides a fallible method to obtain the type of a term.
pub trait OptTyped {
    /// This method returns the type of a term if it is known.
    fn get_type(&self) -> Option<Ty>;
}
