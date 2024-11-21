use printer::{theme::ThemeExt, tokens::SWITCH, util::BracesExt, DocAllocator, Print};

use super::Substitute;
use crate::syntax::{context::context_vars, names::filter_by_set, Clause, Statement, Ty, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::{fresh_var, Linearizing, UsedBinders};
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Switch {
    pub var: Var,
    pub ty: Ty,
    pub clauses: Vec<Clause>,
}

impl Print for Switch {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(SWITCH)
            .append(alloc.space())
            .append(&self.var)
            .append(alloc.space())
            .append(self.clauses.print(cfg, alloc).braces_anno())
    }
}

impl From<Switch> for Statement {
    fn from(value: Switch) -> Self {
        Statement::Switch(value)
    }
}

impl FreeVars for Switch {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.clauses.free_vars(vars);
        vars.insert(self.var.clone());
    }
}

impl Subst for Switch {
    type Target = Switch;

    fn subst_sim(self, subst: &[(Var, Var)]) -> Switch {
        Switch {
            var: self.var.subst_sim(subst),
            clauses: self.clauses.subst_sim(subst),
            ..self
        }
    }
}

impl UsedBinders for Switch {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.clauses.used_binders(used);
    }
}

impl Linearizing for Switch {
    type Target = Statement;
    fn linearize(self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Statement {
        let mut free_vars = HashSet::new();
        self.clauses.free_vars(&mut free_vars);

        let new_context = filter_by_set(&context, &free_vars);
        let mut context_rearrange = new_context.clone();
        context_rearrange.push(self.var.clone());

        // If the condition is true, then `context != context_rearrange`, since then `self.var`
        // is duplicated. Hence, if `context == context_rearrange`, then `var == self.var`.
        let var = if new_context.contains(&self.var) {
            fresh_var(used_vars, &self.var)
        } else {
            self.var.clone()
        };

        let clauses = self
            .clauses
            .into_iter()
            .map(
                |Clause {
                     xtor,
                     context,
                     case,
                 }| {
                    let mut extended_context = new_context.clone();
                    extended_context.append(&mut context_vars(&context));
                    Clause {
                        xtor,
                        context,
                        case: case.linearize(extended_context, used_vars),
                    }
                },
            )
            .collect();
        let switch = Switch {
            var: var.clone(),
            ty: self.ty,
            clauses,
        }
        .into();

        if context == context_rearrange {
            switch
        } else {
            let mut context_rearrange_freshened = new_context.clone();
            context_rearrange_freshened.push(var);

            let rearrange = context_rearrange_freshened
                .into_iter()
                .zip(context_rearrange)
                .collect();
            Substitute {
                rearrange,
                next: Rc::new(switch),
            }
            .into()
        }
    }
}

#[cfg(test)]
mod switch_tests {
    use super::{FreeVars, Linearizing, Subst, Switch, UsedBinders};
    use crate::syntax::{
        clause::Clause,
        context::ContextBinding,
        statements::{Return, Substitute},
        types::Ty,
        Chirality,
    };
    use printer::Print;
    use std::{collections::HashSet, rc::Rc};

    fn example_switch() -> Switch {
        Switch {
            var: "x".to_owned(),
            ty: Ty::Int,
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
                            ty: Ty::Int,
                            chi: Chirality::Prd,
                            var: "x".to_owned(),
                        },
                        ContextBinding {
                            ty: Ty::Decl("ListInt".to_owned()),
                            chi: Chirality::Prd,
                            var: "xs".to_owned(),
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
        }
    }

    #[test]
    fn print_switch() {
        let result = example_switch().print_to_string(Default::default());
        let expected =
            "switch x {Nil() => return y, Cons(x :prd: Int, xs :prd: ListInt) => return x}";
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_switch() {
        let mut result = HashSet::new();
        example_switch().free_vars(&mut result);
        let expected = HashSet::from(["x".to_owned(), "y".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_switch() {
        let result = example_switch().subst_sim(&vec![
            ("x".to_owned(), "a".to_owned()),
            ("y".to_owned(), "b".to_owned()),
        ]);
        let expected = Switch {
            var: "a".to_owned(),
            ty: Ty::Int,
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
                            ty: Ty::Decl("ListInt".to_owned()),
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
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn used_binders_switch() {
        let mut result = HashSet::new();
        example_switch().used_binders(&mut result);
        let expected = HashSet::from(["x".to_owned(), "xs".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn linearize_switch() {
        let result = example_switch().linearize(vec![], &mut HashSet::new());
        let expected = Substitute {
            rearrange: vec![("x".to_owned(), "x".to_owned())],
            next: Rc::new(
                Switch {
                    var: "x".to_owned(),
                    ty: Ty::Int,
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
                                    ty: Ty::Decl("ListInt".to_owned()),
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
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
