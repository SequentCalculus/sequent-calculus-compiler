use core::syntax_var::{statement::Call, TypeDeclaration, Var};

use crate::traits::Shrinking;

use std::collections::HashSet;

impl Shrinking for Call {
    type Target = axcut::syntax::Statement;

    fn shrink(
        self,
        _used_vars: &mut HashSet<Var>,
        _types: &[TypeDeclaration],
    ) -> axcut::syntax::Statement {
        axcut::syntax::Statement::Call(axcut::syntax::statements::Call {
            label: self.name,
            args: self.args,
        })
    }
}
