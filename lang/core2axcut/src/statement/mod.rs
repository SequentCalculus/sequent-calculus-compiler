use crate::traits::{Shrinking, UsedBinders};
use core::syntax_var::{Statement, TypeDeclaration, Var};

use std::collections::HashSet;

pub mod call;
pub mod cut;
pub mod ifz;
pub mod op;

impl UsedBinders for Statement {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        match self {
            Statement::Cut(c) => c.used_binders(used),
            Statement::Op(op) => op.used_binders(used),
            Statement::IfZ(i) => i.used_binders(used),
            _ => {}
        }
    }
}

impl Shrinking for Statement {
    type Target = axcut::syntax::Statement;

    fn shrink(
        self,
        used_vars: &mut HashSet<Var>,
        types: &[TypeDeclaration],
    ) -> axcut::syntax::Statement {
        match self {
            Statement::Cut(cut) => cut.shrink(used_vars, types),
            Statement::Op(op) => op.shrink(used_vars, types),
            Statement::IfZ(ifz) => ifz.shrink(used_vars, types),
            Statement::Call(fun) => fun.shrink(used_vars, types),
            Statement::Done() => axcut::syntax::Statement::Done,
        }
    }
}
