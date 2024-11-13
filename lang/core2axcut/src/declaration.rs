use core::syntax_var::{TypeDeclaration, Var, XtorSig};

use crate::traits::Shrinking;

use std::collections::HashSet;

impl Shrinking for XtorSig {
    type Target = axcut::syntax::XtorSig;

    fn shrink(
        self,
        used_vars: &mut HashSet<Var>,
        types: &[TypeDeclaration],
    ) -> axcut::syntax::XtorSig {
        axcut::syntax::XtorSig {
            name: self.name,
            args: self.args.shrink(used_vars, types),
        }
    }
}

impl Shrinking for TypeDeclaration {
    type Target = axcut::syntax::TypeDeclaration;

    fn shrink(
        self,
        used_vars: &mut HashSet<Var>,
        types: &[TypeDeclaration],
    ) -> axcut::syntax::TypeDeclaration {
        axcut::syntax::TypeDeclaration {
            name: self.name,
            xtors: self.xtors.shrink(used_vars, types),
        }
    }
}
