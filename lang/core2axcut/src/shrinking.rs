//! This module defines a trait for the translation from the focused version of [Core](core_lang)
//! into the non-linearized version of [AxCut](axcut).

use core_lang::syntax::Ident;
use core_lang::syntax::declaration::{CodataDeclaration, DataDeclaration};

use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

/// This struct defines the state of the translation from the focused version of [Core](core_lang)
/// into the non-linearized version of [AxCut](axcut). It consists of the variable names used in
/// the top-level function we are currently in needed for generating fresh variable names, the data
/// and codata type declarations, the labels of top-level functions used in the program and the
/// label of the function we are currently in needed to generate fresh labels, and a list of
/// top-level definitions containing statements within the current function that have been lifted
/// to the top-level.
pub struct ShrinkingState<'a> {
    pub used_vars: &'a mut HashSet<Ident>,
    pub data: &'a [DataDeclaration],
    pub codata: &'a [CodataDeclaration],
    pub used_labels: &'a mut HashSet<Ident>,
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
