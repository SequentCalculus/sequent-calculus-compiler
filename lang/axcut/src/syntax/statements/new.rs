use printer::theme::ThemeExt;
use printer::tokens::{COLON, EQ, NEW, SEMI};
use printer::util::BracesExt;
use printer::{DocAllocator, Print};

use super::Substitute;
use crate::syntax::{
    context::context_vars,
    names::{filter_by_set, freshen},
    Clause, Statement, Ty, Var,
};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::{Linearizing, UsedBinders};
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct New {
    pub var: Var,
    pub ty: Ty,
    pub context: Option<Vec<Var>>,
    pub clauses: Vec<Clause>,
    pub next: Rc<Statement>,
}

impl Print for New {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(NEW)
            .append(alloc.space())
            .append(&self.var)
            .append(alloc.space())
            .append(COLON)
            .append(alloc.space())
            .append(self.ty.print(cfg, alloc))
            .append(alloc.space())
            .append(EQ)
            .append(alloc.space())
            .append(self.context.print(cfg, alloc).parens())
            .append(self.clauses.print(cfg, alloc).braces_anno())
            .append(SEMI)
            .append(alloc.space())
            .append(self.next.print(cfg, alloc))
    }
}

impl From<New> for Statement {
    fn from(value: New) -> Self {
        Statement::New(value)
    }
}

impl FreeVars for New {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.next.free_vars(vars);
        vars.remove(&self.var);
        self.clauses.free_vars(vars);
    }
}

impl Subst for New {
    type Target = New;

    fn subst_sim(self, subst: &[(Var, Var)]) -> New {
        New {
            clauses: self.clauses.subst_sim(subst),
            next: self.next.subst_sim(subst),
            ..self
        }
    }
}

impl UsedBinders for New {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        used.insert(self.var.clone());
        self.clauses.used_binders(used);
        self.next.used_binders(used);
    }
}

impl Linearizing for New {
    type Target = Statement;
    fn linearize(self, mut context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Statement {
        let mut free_vars_clauses = HashSet::new();
        self.clauses.free_vars(&mut free_vars_clauses);
        let mut free_vars_next = HashSet::new();
        self.next.free_vars(&mut free_vars_next);

        let context_clone = context.clone();
        let mut context_next = filter_by_set(&context, &free_vars_next);
        // This splitting and reordering is just to retain as many positions as possible.
        let mut context_reordered = context.split_off(context_next.len());
        context_reordered.append(&mut context);
        let context_clauses = filter_by_set(&context_reordered, &free_vars_clauses);

        let clauses = self
            .clauses
            .into_iter()
            .map(
                |Clause {
                     xtor,
                     context,
                     case,
                 }| {
                    let mut extended_context = context_vars(&context);
                    extended_context.append(&mut context_clauses.clone());
                    Clause {
                        xtor,
                        context,
                        case: case.linearize(extended_context, used_vars),
                    }
                },
            )
            .collect();

        let mut context_rearrange = context_next.clone();
        context_rearrange.append(&mut context_clauses.clone());

        if context_clone == context_rearrange {
            context_next.push(self.var.clone());
            New {
                var: self.var,
                ty: self.ty,
                context: Some(context_clauses),
                clauses,
                next: self.next.linearize(context_next, used_vars),
            }
            .into()
        } else {
            let mut context_next_freshened = freshen(
                &context_next,
                context_clauses.clone().into_iter().collect(),
                used_vars,
            );
            let mut context_rearrange_freshened = context_next_freshened.clone();
            context_rearrange_freshened.append(&mut context_clauses.clone());

            let rearrange = context_rearrange_freshened
                .into_iter()
                .zip(context_rearrange)
                .collect();

            let substitution_next: Vec<(Var, Var)> = context_next
                .into_iter()
                .zip(context_next_freshened.clone())
                .collect();
            let next_substituted = self.next.subst_sim(substitution_next.as_slice());

            context_next_freshened.push(self.var.clone());
            Substitute {
                rearrange,
                next: Rc::new(
                    New {
                        var: self.var,
                        ty: self.ty,
                        context: Some(context_clauses),
                        clauses,
                        next: next_substituted.linearize(context_next_freshened, used_vars),
                    }
                    .into(),
                ),
            }
            .into()
        }
    }
}

#[cfg(test)]
mod new_tests {
    use super::{FreeVars, Linearizing, New, Subst, UsedBinders};
    use crate::syntax::{
        clause::Clause,
        context::ContextBinding,
        statements::{Return, Substitute},
        types::Ty,
        Chirality,
    };
    use printer::Print;
    use std::{collections::HashSet, rc::Rc};

    fn example_new() -> New {
        New {
            var: "x".to_owned(),
            ty: Ty::Int,
            context: Some(vec!["y".to_owned(), "z".to_owned()]),
            clauses: vec![
                Clause {
                    xtor: "Nil".to_owned(),
                    context: vec![],
                    case: Rc::new(
                        Return {
                            var: "y".to_owned(),
                        }
                        .into(),
                    ),
                },
                Clause {
                    xtor: "Cons".to_owned(),
                    context: vec![
                        ContextBinding {
                            var: "x".to_owned(),
                            chi: Chirality::Prd,
                            ty: Ty::Int,
                        },
                        ContextBinding {
                            var: "xs".to_owned(),
                            chi: Chirality::Prd,
                            ty: Ty::Int,
                        },
                    ],
                    case: Rc::new(
                        Return {
                            var: "x".to_owned(),
                        }
                        .into(),
                    ),
                },
            ],
            next: Rc::new(
                Return {
                    var: "y".to_owned(),
                }
                .into(),
            ),
        }
    }

    #[test]
    fn print_new() {
        let result = example_new().print_to_string(Default::default());
        let expected = "new x : Int = (y, z){Nil() => return y, Cons(x :prd: Int, xs :prd: Int) => return x}; return y";
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_new() {
        let mut result = HashSet::new();
        example_new().free_vars(&mut result);
        let expected = HashSet::from(["y".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_new() {
        let result = example_new().subst_sim(&vec![
            ("x".to_owned(), "a".to_owned()),
            ("y".to_owned(), "b".to_owned()),
            ("z".to_owned(), "c".to_owned()),
        ]);
        let expected = New {
            var: "x".to_owned(),
            ty: Ty::Int,
            context: Some(vec!["y".to_owned(), "z".to_owned()]),
            clauses: vec![
                Clause {
                    xtor: "Nil".to_owned(),
                    context: vec![],
                    case: Rc::new(
                        Return {
                            var: "b".to_owned(),
                        }
                        .into(),
                    ),
                },
                Clause {
                    xtor: "Cons".to_owned(),
                    context: vec![
                        ContextBinding {
                            var: "x".to_owned(),
                            chi: Chirality::Prd,
                            ty: Ty::Int,
                        },
                        ContextBinding {
                            var: "xs".to_owned(),
                            chi: Chirality::Prd,
                            ty: Ty::Int,
                        },
                    ],
                    case: Rc::new(
                        Return {
                            var: "a".to_owned(),
                        }
                        .into(),
                    ),
                },
            ],
            next: Rc::new(
                Return {
                    var: "b".to_owned(),
                }
                .into(),
            ),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn used_binders_new() {
        let mut result = HashSet::new();
        example_new().used_binders(&mut result);
        let expected = HashSet::from(["x".to_owned(), "xs".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn linearize_new() {
        let result = example_new().linearize(vec![], &mut HashSet::new());
        let expected = Substitute {
            rearrange: vec![],
            next: Rc::new(
                New {
                    var: "x".to_owned(),
                    ty: Ty::Int,
                    context: Some(vec![]),
                    clauses: vec![
                        Clause {
                            xtor: "Nil".to_owned(),
                            context: vec![],
                            case: Rc::new(
                                Return {
                                    var: "y".to_owned(),
                                }
                                .into(),
                            ),
                        },
                        Clause {
                            xtor: "Cons".to_owned(),
                            context: vec![
                                ContextBinding {
                                    var: "x".to_owned(),
                                    chi: Chirality::Prd,
                                    ty: Ty::Int,
                                },
                                ContextBinding {
                                    var: "xs".to_owned(),
                                    chi: Chirality::Prd,
                                    ty: Ty::Int,
                                },
                            ],
                            case: Rc::new(
                                Return {
                                    var: "x".to_owned(),
                                }
                                .into(),
                            ),
                        },
                    ],
                    next: Rc::new(
                        Return {
                            var: "y".to_owned(),
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
