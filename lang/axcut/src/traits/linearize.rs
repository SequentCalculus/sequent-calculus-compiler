//! This module defines a trait with a method for performing the linearization pass translating
//! the non-linearized version of AxCut into the linearized one.

use crate::syntax::TypingContext;

use std::rc::Rc;

/// This trait defines a method for linearizing a statement, translating the non-linearized
/// statement into the linearized version.
pub trait Linearizing {
    type Target;
    /// This method linearizes a statement, translating the given non-linearized version into
    /// the linearized one. It inserts an explicit substitution before most statements which takes
    /// care of adapting the context appropriately. It assumes all variable bindings in each path
    /// through the statement to be unique and maintains this invariant.
    /// - `context` is the list of variables currently in the environment. It constitutes the
    ///   type environment the given statement is supposed to be typed in.
    ///   linearized. It is threaded through the linearization to facilitate generation of fresh
    ///   variables.
    /// - `max_id` is the highest used id for identifiers. It is incremented whenever a new
    ///   identifier is created
    fn linearize(self, context: TypingContext, max_id: &mut usize) -> Self::Target;
}

impl<T: Linearizing + Clone> Linearizing for Rc<T> {
    type Target = Rc<T::Target>;
    fn linearize(self, context: TypingContext, max_id: &mut usize) -> Self::Target {
        Rc::new(Rc::unwrap_or_clone(self).linearize(context, max_id))
    }
}
