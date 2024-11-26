use crate::traits::Shrinking;
use core::syntax_var::{FsStatement, FsTypeDeclaration, Var};

use std::collections::HashSet;

pub mod call;
pub mod cut;
pub mod ife;
pub mod ifl;
pub mod ifz;
pub mod op;

impl Shrinking for FsStatement {
    type Target = axcut::syntax::Statement;

    fn shrink(
        self,
        used_vars: &mut HashSet<Var>,
        types: &[FsTypeDeclaration],
    ) -> axcut::syntax::Statement {
        match self {
            FsStatement::Cut(cut) => cut.shrink(used_vars, types),
            FsStatement::Op(op) => op.shrink(used_vars, types),
            FsStatement::IfE(ife) => ife.shrink(used_vars, types),
            FsStatement::IfL(ifl) => ifl.shrink(used_vars, types),
            FsStatement::IfZ(ifz) => ifz.shrink(used_vars, types),
            FsStatement::Call(fun) => fun.shrink(used_vars, types),
            FsStatement::Done() => axcut::syntax::Statement::Done,
        }
    }
}
