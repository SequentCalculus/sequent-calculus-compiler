//! This module defines a trait for the translation from the focused version of [Core](core_lang)
//! into the non-linearized version of [AxCut](axcut).

use core_lang::syntax::declaration::{CodataDeclaration, DataDeclaration};
use core_lang::syntax::{Name, Var};

use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

/// This struct defines the state of the translation from the focused version of [Core](core_lang)
/// into the non-linearized version of [AxCut](axcut). It consists of the variable names used in
/// the top-level function we are currently in which we need for generating fresh variable names,
/// the data and codata type declarations, the labels of top-level functions used in the program
/// and the label of the function we are currently in which we need to generate fresh labels, and a
/// list of statements within the current function that are lifted to the top-level.
pub struct ShrinkingState<'a> {
    pub used_vars: &'a mut HashSet<Var>,
    pub data: &'a [DataDeclaration],
    pub codata: &'a [CodataDeclaration],
    pub used_labels: &'a mut HashSet<Name>,
    pub current_label: &'a str,
    pub lifted_statements: &'a mut VecDeque<axcut::syntax::Def>,
}

/// This trait provides a method for the translation from the focused version of [Core](core_lang)
/// into the non-linearized version of [AxCut](axcut).
pub trait Shrinking {
    type Target;
    /// This method translates a term from the focused version of [Core](core_lang) into the
    /// non-linearized version of [AxCut](axcut). It only operates on statements of
    /// [Core](core_lang) by inlining producers and consumers into cuts and then eliminates
    /// several redundant statement forms, thus shrinking the language. At the same time it
    /// collapses the resulting statements to the one-sided statements of [AxCut](axcut).
    /// The method assumes all variable bindings in each path through the statements to be unique
    /// and maintains this invariant.
    /// - `state` is the [state](ShrinkingState) threaded through the translation.
    fn shrink(self, state: &mut ShrinkingState) -> Self::Target;
}

impl<T: Shrinking + Clone> Shrinking for Rc<T> {
    type Target = Rc<T::Target>;
    fn shrink(self, state: &mut ShrinkingState) -> Self::Target {
        Rc::new(Rc::unwrap_or_clone(self).shrink(state))
    }
}

impl<T: Shrinking> Shrinking for Vec<T> {
    type Target = Vec<T::Target>;
    fn shrink(self, state: &mut ShrinkingState) -> Self::Target {
        self.into_iter()
            .map(|element| element.shrink(state))
            .collect()
    }
}
