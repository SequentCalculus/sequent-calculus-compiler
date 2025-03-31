use core_lang::syntax::declaration::{CodataDeclaration, DataDeclaration};
use core_lang::syntax::{Name, Var};

use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

pub struct ShrinkingState<'a> {
    pub used_vars: &'a mut HashSet<Var>,
    pub data: &'a [DataDeclaration],
    pub codata: &'a [CodataDeclaration],
    pub used_labels: &'a mut HashSet<Name>,
    pub current_label: &'a str,
    pub lifted_statements: &'a mut VecDeque<axcut::syntax::Def>,
}

/// This assumes all variable bindings to be unique and maintains this invariant.
pub trait Shrinking {
    type Target;
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
