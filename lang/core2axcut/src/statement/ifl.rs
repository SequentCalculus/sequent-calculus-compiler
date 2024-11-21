use core::syntax_var::{statement::IfL, TypeDeclaration, Var};

use crate::traits::Shrinking;

use std::collections::HashSet;

impl Shrinking for IfL {
    type Target = axcut::syntax::Statement;

    fn shrink(
        self,
        used_vars: &mut HashSet<Var>,
        types: &[TypeDeclaration],
    ) -> axcut::syntax::Statement {
        axcut::syntax::Statement::IfL(axcut::syntax::statements::IfL {
            fst: self.fst,
            snd: self.snd,
            thenc: self.thenc.shrink(used_vars, types),
            elsec: self.elsec.shrink(used_vars, types),
        })
    }
}

#[cfg(test)]
mod ifl_tests {
    use super::Shrinking;
    use std::{collections::HashSet, rc::Rc};

    #[test]
    fn shrink_ifl() {
        let result = core::syntax_var::statement::IfL {
            fst: "x".to_owned(),
            snd: "y".to_owned(),
            thenc: Rc::new(core::syntax_var::Statement::Done()),
            elsec: Rc::new(core::syntax_var::Statement::Done()),
        }
        .shrink(&mut HashSet::new(), &vec![]);
        let expected = axcut::syntax::statements::IfL {
            fst: "x".to_owned(),
            snd: "y".to_owned(),
            thenc: Rc::new(axcut::syntax::Statement::Done),
            elsec: Rc::new(axcut::syntax::Statement::Done),
        }
        .into();
        assert_eq!(result, expected)
    }
}
