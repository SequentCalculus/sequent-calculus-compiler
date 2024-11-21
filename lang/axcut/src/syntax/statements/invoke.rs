use printer::{theme::ThemeExt, tokens::INVOKE, DocAllocator, Print};

use super::Substitute;
use crate::syntax::{names::freshen, Name, Statement, Ty, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::Linearizing;
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invoke {
    pub var: Var,
    pub tag: Name,
    pub ty: Ty,
    pub args: Vec<Var>,
}

impl Print for Invoke {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let args = if self.args.is_empty() {
            alloc.nil()
        } else {
            self.args.print(cfg, alloc).parens()
        };
        alloc
            .keyword(INVOKE)
            .append(alloc.space())
            .append(&self.var)
            .append(alloc.space())
            .append(&self.tag)
            .append(args)
    }
}

impl From<Invoke> for Statement {
    fn from(value: Invoke) -> Self {
        Statement::Invoke(value)
    }
}

impl FreeVars for Invoke {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.args.free_vars(vars);
        vars.insert(self.var.clone());
    }
}

impl Subst for Invoke {
    type Target = Invoke;

    fn subst_sim(self, subst: &[(Var, Var)]) -> Invoke {
        Invoke {
            var: self.var.subst_sim(subst),
            args: self.args.subst_sim(subst),
            ..self
        }
    }
}

impl Linearizing for Invoke {
    type Target = Statement;
    fn linearize(self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Statement {
        let invoke = Invoke {
            var: self.var.clone(),
            tag: self.tag,
            ty: self.ty,
            args: vec![],
        }
        .into();

        let mut context_rearrange = self.args.clone();
        context_rearrange.push(self.var.clone());

        if context == context_rearrange {
            invoke
        } else {
            let mut freshened_context = freshen(&self.args, HashSet::new(), used_vars);
            freshened_context.push(self.var);

            let rearrange: Vec<(Var, Var)> = freshened_context
                .into_iter()
                .zip(context_rearrange)
                .collect();
            Substitute {
                rearrange,
                next: Rc::new(invoke),
            }
            .into()
        }
    }
}

#[cfg(test)]
mod invoke_tests {
    use super::{FreeVars, Invoke, Linearizing, Subst};
    use crate::syntax::{statements::Substitute, types::Ty};
    use printer::Print;
    use std::{collections::HashSet, rc::Rc};

    fn example_invoke() -> Invoke {
        Invoke {
            var: "x".to_owned(),
            tag: "main".to_owned(),
            ty: Ty::Int,
            args: vec!["y".to_owned(), "z".to_owned()],
        }
    }

    #[test]
    fn print_invoke() {
        let result = example_invoke().print_to_string(Default::default());
        let expected = "invoke x main(y, z)";
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_invoke() {
        let mut result = HashSet::new();
        example_invoke().free_vars(&mut result);
        let expected = HashSet::from(["x".to_owned(), "y".to_owned(), "z".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_invoke() {
        let result = example_invoke().subst_sim(&vec![
            ("x".to_owned(), "a".to_owned()),
            ("y".to_owned(), "b".to_owned()),
            ("z".to_owned(), "c".to_owned()),
        ]);
        let expected = Invoke {
            var: "a".to_owned(),
            tag: "main".to_owned(),
            ty: Ty::Int,
            args: vec!["b".to_owned(), "c".to_owned()],
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn linearize_invoke() {
        let result = example_invoke().linearize(vec![], &mut HashSet::new());
        let expected = Substitute {
            rearrange: vec![
                ("y".to_owned(), "y".to_owned()),
                ("z".to_owned(), "z".to_owned()),
                ("x".to_owned(), "x".to_owned()),
            ],
            next: Rc::new(
                Invoke {
                    var: "x".to_owned(),
                    tag: "main".to_owned(),
                    ty: Ty::Int,
                    args: vec![],
                }
                .into(),
            ),
        }
        .into();

        assert_eq!(result, expected)
    }
}
