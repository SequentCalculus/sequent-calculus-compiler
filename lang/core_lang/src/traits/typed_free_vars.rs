use crate::syntax::{CodataDeclaration, ContextBinding, DataDeclaration, Name, TypingContext};

use std::collections::{BTreeSet, HashMap};

pub struct TypedFreeVarsState<'a> {
    pub data: &'a [DataDeclaration],
    pub codata: &'a [CodataDeclaration],
    pub def_signatures: &'a HashMap<Name, TypingContext>,
}

/// Computing the typed free variables of a term.
pub trait TypedFreeVars: Sized {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>, state: &TypedFreeVarsState);
}

impl<T: TypedFreeVars> TypedFreeVars for Vec<T> {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>, state: &TypedFreeVarsState) {
        for element in self {
            element.typed_free_vars(vars, state);
        }
    }
}
