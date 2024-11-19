use crate::traits::Shrinking;
use core::syntax_var::{Statement, TypeDeclaration, Var};

use std::collections::HashSet;

pub mod call;
pub mod cut;
pub mod ife;
pub mod ifl;
pub mod ifz;
pub mod op;

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
            Statement::IfE(ife) => ife.shrink(used_vars, types),
            Statement::IfL(ifl) => ifl.shrink(used_vars, types),
            Statement::IfZ(ifz) => ifz.shrink(used_vars, types),
            Statement::Call(fun) => fun.shrink(used_vars, types),
            Statement::Done() => axcut::syntax::Statement::Done,
        }
    }
}
