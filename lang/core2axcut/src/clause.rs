use core::syntax::clause::FsClause;
use core::syntax_var::{FsTypeDeclaration, Var};

use crate::context::translate_context;
use crate::traits::Shrinking;

use std::collections::HashSet;

impl Shrinking for FsClause {
    type Target = axcut::syntax::Clause;

    fn shrink(
        self,
        used_vars: &mut HashSet<Var>,
        types: &[FsTypeDeclaration],
    ) -> axcut::syntax::Clause {
        axcut::syntax::Clause {
            xtor: self.xtor,
            context: translate_context(self.context),
            case: self.case.shrink(used_vars, types),
        }
    }
}
