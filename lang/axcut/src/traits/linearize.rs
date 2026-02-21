//! This module defines a trait with a method for performing the linearization pass translating
//! the non-linearized version of AxCut into the linearized one.

use crate::syntax::{Ident, TypingContext};

use std::collections::HashSet;
use std::rc::Rc;

/// This function generates a fresh variable with respect to a given set of variable names.
/// - `used_vars` is the set of variable names for which to generate a fresh one.
/// - `base_name` is the base name for the generated variable to which a number is appended that
///   makes it fresh.
pub fn fresh_var(used_vars: &mut HashSet<Ident>, base_name: &str) -> Ident {
    let mut new_var = Ident {
        name: base_name.to_string(),
        id: 0,
    };
    while used_vars.contains(&new_var) {
        new_var.id += 1;
    }
    used_vars.insert(new_var.clone());
    new_var
}

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
    /// - `used_vars` is the set of variable names used in the whole top-level definition being
    ///   linearized. It is threaded through the linearization to facilitate generation of fresh
    ///   variables.
    fn linearize(self, context: TypingContext, used_vars: &mut HashSet<Ident>) -> Self::Target;
}

impl<T: Linearizing + Clone> Linearizing for Rc<T> {
    type Target = Rc<T::Target>;
    fn linearize(self, context: TypingContext, used_vars: &mut HashSet<Ident>) -> Self::Target {
        Rc::new(Rc::unwrap_or_clone(self).linearize(context, used_vars))
    }
}
