use core::syntax_var::{statement::IfZ, TypeDeclaration, Var};

use crate::traits::Shrinking;

use std::collections::HashSet;

impl Shrinking for IfZ {
    type Target = axcut::syntax::Statement;

    fn shrink(
        self,
        used_vars: &mut HashSet<Var>,
        types: &[TypeDeclaration],
    ) -> axcut::syntax::Statement {
        axcut::syntax::Statement::IfZ(axcut::syntax::statements::IfZ {
            ifc: self.ifc,
            thenc: self.thenc.shrink(used_vars, types),
            elsec: self.elsec.shrink(used_vars, types),
        })
    }
}

#[cfg(test)]
mod ifz_tests {
    use super::Shrinking;
    use std::{collections::HashSet, rc::Rc};

    #[test]
    fn shrink_ifz() {
        let result = core::syntax_var::statement::IfZ {
            ifc: "x".to_owned(),
            thenc: Rc::new(core::syntax_var::Statement::Done()),
            elsec: Rc::new(core::syntax_var::Statement::Done()),
        }
        .shrink(&mut HashSet::new(), &vec![]);
        let expected = axcut::syntax::statements::IfZ {
            ifc: "x".to_owned(),
            thenc: Rc::new(axcut::syntax::Statement::Done),
            elsec: Rc::new(axcut::syntax::Statement::Done),
        }
        .into();
        assert_eq!(result, expected)
    }
}
