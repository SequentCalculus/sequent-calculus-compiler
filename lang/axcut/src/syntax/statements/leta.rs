use printer::theme::ThemeExt;
use printer::tokens::{COLON, EQ, LETA, SEMI};
use printer::{DocAllocator, Print};

use super::Substitute;
use crate::syntax::{
    names::{filter_by_set, freshen},
    Name, Statement, Ty, Var,
};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::{Linearizing, UsedBinders};
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Leta {
    pub var: Var,
    pub ty: Ty,
    pub tag: Name,
    pub args: Vec<Var>,
    pub next: Rc<Statement>,
}

impl Print for Leta {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(LETA)
            .append(alloc.space())
            .append(&self.var)
            .append(COLON)
            .append(alloc.space())
            .append(self.ty.print(cfg, alloc))
            .append(alloc.space())
            .append(EQ)
            .append(alloc.space())
            .append(&self.tag)
            .append(self.args.print(cfg, alloc).parens())
            .append(SEMI)
            .append(alloc.space())
            .append(self.next.print(cfg, alloc))
    }
}

impl From<Leta> for Statement {
    fn from(value: Leta) -> Self {
        Statement::Leta(value)
    }
}

impl FreeVars for Leta {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.next.free_vars(vars);
        vars.remove(&self.var);
        self.args.free_vars(vars);
    }
}

impl Subst for Leta {
    type Target = Leta;

    fn subst_sim(self, subst: &[(Var, Var)]) -> Leta {
        Leta {
            args: self.args.subst_sim(subst),
            next: self.next.subst_sim(subst),
            ..self
        }
    }
}

impl UsedBinders for Leta {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        used.insert(self.var.clone());
        self.next.used_binders(used);
    }
}

impl Linearizing for Leta {
    type Target = Statement;
    fn linearize(self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Statement {
        let mut free_vars = HashSet::new();
        self.next.free_vars(&mut free_vars);

        let mut new_context = filter_by_set(&context, &free_vars);

        let mut context_rearrange = new_context.clone();
        context_rearrange.append(&mut self.args.clone());

        if context == context_rearrange {
            new_context.push(self.var.clone());
            Leta {
                var: self.var,
                ty: self.ty,
                tag: self.tag,
                args: self.args,
                next: self.next.linearize(new_context, used_vars),
            }
            .into()
        } else {
            let freshened_context = freshen(
                &self.args,
                new_context.clone().into_iter().collect(),
                used_vars,
            );

            let mut context_rearrange_freshened = new_context.clone();
            context_rearrange_freshened.append(&mut freshened_context.clone());

            let rearrange = context_rearrange_freshened
                .into_iter()
                .zip(context_rearrange)
                .collect();
            new_context.push(self.var.clone());
            Substitute {
                rearrange,
                next: Rc::new(
                    Leta {
                        var: self.var,
                        ty: self.ty,
                        tag: self.tag,
                        args: freshened_context,
                        next: self.next.linearize(new_context, used_vars),
                    }
                    .into(),
                ),
            }
            .into()
        }
    }
}

#[cfg(test)]
mod leta_tests {
    use super::{FreeVars, Leta, Linearizing, Subst, UsedBinders};
    use crate::syntax::{
        statements::{Return, Substitute},
        types::Ty,
    };
    use printer::Print;
    use std::{collections::HashSet, rc::Rc};

    fn example_leta() -> Leta {
        Leta {
            var: "x".to_owned(),
            ty: Ty::Int,
            tag: "main".to_owned(),
            args: vec!["y".to_owned(), "z".to_owned()],
            next: Rc::new(
                Return {
                    var: "a".to_owned(),
                }
                .into(),
            ),
        }
    }

    #[test]
    fn print_leta() {
        let result = example_leta().print_to_string(Default::default());
        let expected = "leta x: Int = main(y, z); return a";
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_leta() {
        let mut result = HashSet::new();
        example_leta().free_vars(&mut result);
        let expected = HashSet::from(["y".to_owned(), "z".to_owned(), "a".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_leta() {
        let result = example_leta().subst_sim(&vec![
            ("x".to_owned(), "a".to_owned()),
            ("y".to_owned(), "b".to_owned()),
            ("z".to_owned(), "c".to_owned()),
        ]);
        let expected = Leta {
            var: "x".to_owned(),
            ty: Ty::Int,
            tag: "main".to_owned(),
            args: vec!["b".to_owned(), "c".to_owned()],
            next: Rc::new(
                Return {
                    var: "a".to_owned(),
                }
                .into(),
            ),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn used_binders_leta() {
        let mut result = HashSet::new();
        example_leta().used_binders(&mut result);
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn linearize_leta() {
        let result = example_leta().linearize(vec![], &mut HashSet::new());
        let expected = Substitute {
            rearrange: vec![
                ("y".to_owned(), "y".to_owned()),
                ("z".to_owned(), "z".to_owned()),
            ],
            next: Rc::new(
                Leta {
                    var: "x".to_owned(),
                    ty: Ty::Int,
                    tag: "main".to_owned(),
                    args: vec!["y".to_owned(), "z".to_owned()],
                    next: Rc::new(
                        Return {
                            var: "a".to_owned(),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
