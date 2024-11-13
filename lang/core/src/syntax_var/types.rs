use super::{Name, TypeDeclaration, Var};
use crate::traits::shrink::Shrinking;

use std::collections::HashSet;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Ty {
    Int,
    Decl(Name),
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Ty::Int => f.write_str("Int"),
            Ty::Decl(name) => f.write_str(name),
        }
    }
}

impl Shrinking for Ty {
    type Target = axcut::syntax::Ty;

    fn shrink(
        self,
        _used_vars: &mut HashSet<Var>,
        _types: &[TypeDeclaration],
    ) -> axcut::syntax::Ty {
        match self {
            Ty::Int => axcut::syntax::Ty::Int,
            Ty::Decl(name) => axcut::syntax::Ty::Decl(name),
        }
    }
}
