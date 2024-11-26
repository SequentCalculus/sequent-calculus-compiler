use core::{
    syntax::{statement::FsCall, Var},
    syntax_var::FsTypeDeclaration,
};

use crate::traits::Shrinking;

use std::collections::HashSet;

impl Shrinking for FsCall {
    type Target = axcut::syntax::Statement;

    fn shrink(
        self,
        _used_vars: &mut HashSet<Var>,
        _types: &[FsTypeDeclaration],
    ) -> axcut::syntax::Statement {
        axcut::syntax::Statement::Call(axcut::syntax::statements::Call {
            label: self.name,
            args: self.args,
        })
    }
}
