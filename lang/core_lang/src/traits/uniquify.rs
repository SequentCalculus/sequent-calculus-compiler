//! This module defines a trait for making all binders in every path through a term or statement
//! unique.

use crate::syntax::ID;
use std::rc::Rc;

/// This trait defines a method for making all binders in every path through a term or statement
/// unique.
pub trait Uniquify {
    /// This method makes all binders in a term or statement unique for the program assigning a
    /// globally unique ID.
    /// - `max_id` is the highest [`ID`] currently used for [`crate::syntax::Identifier`]s in the
    ///   program. It is threaded through the uniquifying to facilitate generation of fresh
    ///   (co)variables.
    fn uniquify(self, max_id: &mut ID) -> Self;
}

impl<T: Uniquify + Clone> Uniquify for Rc<T> {
    fn uniquify(self, max_id: &mut ID) -> Self {
        Rc::new(Rc::unwrap_or_clone(self).uniquify(max_id))
    }
}

impl<T: Uniquify> Uniquify for Option<T> {
    fn uniquify(self, max_id: &mut ID) -> Self {
        self.map(|t| t.uniquify(max_id))
    }
}

impl<T: Uniquify> Uniquify for Vec<T> {
    fn uniquify(self, max_id: &mut ID) -> Self {
        self.into_iter()
            .map(|element| element.uniquify(max_id))
            .collect()
    }
}
