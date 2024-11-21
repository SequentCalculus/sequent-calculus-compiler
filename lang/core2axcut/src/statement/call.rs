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

#[cfg(test)]
mod call_tests {
    use super::Shrinking;
    use std::collections::HashSet;

    #[test]
    fn shrink_call() {
        let result = core::syntax_var::statement::Call {
            name: "exit".to_owned(),
            args: vec![],
        }
        .shrink(&mut HashSet::new(), &vec![]);
        let expected = axcut::syntax::statements::Call {
            label: "exit".to_owned(),
            args: vec![],
        }
        .into();
        assert_eq!(result, expected)
    }
}
