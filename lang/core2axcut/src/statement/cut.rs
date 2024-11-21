use core::traits::substitution::SubstVar;
use core::{
    syntax_var::term::{Literal, Mu, Term, XCase, XVar, Xtor},
    syntax_var::{
        cont_int,
        context::context_vars,
        declaration::lookup_type_declaration,
        statement::Cut,
        Chirality::{Cns, Prd},
        Clause, Name, Statement, Ty, TypeDeclaration, Var,
    },
    traits::free_vars::fresh_var,
};

use crate::chirality::translate_chirality;
use crate::traits::Shrinking;
use crate::types::translate_ty;

use std::{collections::HashSet, rc::Rc};

fn shrink_renaming(
    var: Var,
    var_mu: Var,
    statement: Rc<Statement>,
    ty: &Ty,
    used_vars: &mut HashSet<Var>,
    types: &[TypeDeclaration],
) -> axcut::syntax::Statement {
    if *ty == Ty::Int && *statement == Statement::Done() {
        axcut::syntax::Statement::Return(axcut::syntax::statements::Return { var })
    } else {
        Rc::unwrap_or_clone(statement)
            .subst_sim(&[(var_mu, var)])
            .shrink(used_vars, types)
    }
}

fn shrink_known_cuts(
    id: &Name,
    args: Vec<Var>,
    clauses: &[Clause],
    used_vars: &mut HashSet<Var>,
    types: &[TypeDeclaration],
) -> axcut::syntax::Statement {
    let (statement, context) = match clauses.iter().find(
        |Clause {
             xtor,
             context: _,
             case: _,
         }| xtor == id,
    ) {
        None => panic!("Xtor {id} not found in clauses {clauses:?}"),
        Some(Clause {
            xtor: _,
            context,
            case,
        }) => (case.clone(), context),
    };
    let subst: Vec<(Var, Var)> = context_vars(context).into_iter().zip(args).collect();
    Rc::unwrap_or_clone(statement)
        .subst_sim(subst.as_slice())
        .shrink(used_vars, types)
}

fn shrink_unknown_cuts(
    var_prd: Var,
    var_cns: Var,
    ty: Ty,
    used_vars: &mut HashSet<Var>,
    types: &[TypeDeclaration],
) -> axcut::syntax::Statement {
    match ty.clone() {
        Ty::Int => axcut::syntax::Statement::Invoke(axcut::syntax::statements::Invoke {
            var: var_cns,
            tag: cont_int().xtors[0].name.clone(),
            ty: axcut::syntax::Ty::Decl(cont_int().name),
            args: vec![var_prd],
        }),
        Ty::Decl(name) => {
            let type_declaration = lookup_type_declaration(&name, types);
            let clauses: Vec<axcut::syntax::Clause> = type_declaration
                .xtors
                .iter()
                .map(|xtor| {
                    let env: Vec<axcut::syntax::ContextBinding> = xtor
                        .args
                        .iter()
                        .map(|arg| axcut::syntax::ContextBinding {
                            var: fresh_var(used_vars, &arg.var),
                            chi: translate_chirality(&arg.chi.clone()),
                            ty: translate_ty(arg.ty.clone()),
                        })
                        .collect();
                    axcut::syntax::Clause {
                        xtor: xtor.name.clone(),
                        context: env.clone(),
                        case: Rc::new(axcut::syntax::Statement::Invoke(
                            axcut::syntax::statements::Invoke {
                                var: var_cns.clone(),
                                tag: xtor.name.clone(),
                                ty: translate_ty(ty.clone()),
                                args: axcut::syntax::context::context_vars(&env),
                            },
                        )),
                    }
                })
                .collect();
            axcut::syntax::Statement::Switch(axcut::syntax::statements::Switch {
                var: var_prd,
                ty: translate_ty(ty),
                clauses,
            })
        }
    }
}

fn shrink_critical_pairs(
    var_prd: Var,
    statement_prd: Rc<Statement>,
    var_cns: Var,
    statement_cns: Rc<Statement>,
    ty: &Ty,
    used_vars: &mut HashSet<Var>,
    types: &[TypeDeclaration],
) -> axcut::syntax::Statement {
    match ty.clone() {
        Ty::Int => {
            let case = if *statement_cns == Statement::Done() {
                Rc::new(axcut::syntax::Statement::Return(
                    axcut::syntax::statements::Return {
                        var: var_cns.clone(),
                    },
                ))
            } else {
                statement_cns.shrink(used_vars, types)
            };
            axcut::syntax::Statement::New(axcut::syntax::statements::New {
                var: var_prd,
                ty: axcut::syntax::Ty::Decl(cont_int().name),
                context: None,
                clauses: vec![axcut::syntax::Clause {
                    xtor: cont_int().xtors[0].name.clone(),
                    context: vec![axcut::syntax::ContextBinding {
                        var: var_cns,
                        chi: axcut::syntax::Chirality::Ext,
                        ty: axcut::syntax::Ty::Int,
                    }],
                    case,
                }],
                next: statement_prd.shrink(used_vars, types),
            })
        }
        Ty::Decl(name) => {
            let type_declaration = lookup_type_declaration(&name, types);
            let clauses: Vec<axcut::syntax::Clause> = type_declaration
                .xtors
                .iter()
                .map(|xtor| {
                    let env: Vec<axcut::syntax::ContextBinding> = xtor
                        .args
                        .iter()
                        .map(|arg| axcut::syntax::ContextBinding {
                            var: fresh_var(used_vars, &arg.var),
                            chi: translate_chirality(&arg.chi.clone()),
                            ty: translate_ty(arg.ty.clone()),
                        })
                        .collect();
                    axcut::syntax::Clause {
                        xtor: xtor.name.clone(),
                        context: env.clone(),
                        case: Rc::new(axcut::syntax::Statement::Leta(
                            axcut::syntax::statements::Leta {
                                var: var_cns.clone(),
                                ty: translate_ty(ty.clone()),
                                tag: xtor.name.clone(),
                                args: axcut::syntax::context::context_vars(&env),
                                next: statement_cns.clone().shrink(used_vars, types),
                            },
                        )),
                    }
                })
                .collect();
            axcut::syntax::Statement::New(axcut::syntax::statements::New {
                var: var_prd,
                ty: axcut::syntax::Ty::Decl(name),
                context: None,
                clauses,
                next: statement_prd.shrink(used_vars, types),
            })
        }
    }
}

fn shrink_literal_mu(
    lit: i64,
    var: Var,
    statement: Rc<Statement>,
    used_vars: &mut HashSet<Var>,
    types: &[TypeDeclaration],
) -> axcut::syntax::Statement {
    let case = if *statement == Statement::Done() {
        Rc::new(axcut::syntax::Statement::Return(
            axcut::syntax::statements::Return { var: var.clone() },
        ))
    } else {
        statement.shrink(used_vars, types)
    };
    axcut::syntax::Statement::Literal(axcut::syntax::statements::Literal { lit, var, case })
}

fn shrink_literal_var(
    lit: i64,
    var: Var,
    used_vars: &mut HashSet<Var>,
) -> axcut::syntax::Statement {
    let fresh_var = fresh_var(used_vars, "x");
    axcut::syntax::Statement::Literal(axcut::syntax::statements::Literal {
        lit,
        var: fresh_var.clone(),
        case: Rc::new(axcut::syntax::Statement::Invoke(
            axcut::syntax::statements::Invoke {
                var,
                tag: cont_int().xtors[0].name.clone(),
                ty: axcut::syntax::Ty::Decl(cont_int().name),
                args: vec![fresh_var],
            },
        )),
    })
}

impl Shrinking for Cut {
    type Target = axcut::syntax::Statement;

    fn shrink(
        self,
        used_vars: &mut HashSet<Var>,
        types: &[TypeDeclaration],
    ) -> axcut::syntax::Statement {
        match (
            Rc::unwrap_or_clone(self.producer),
            Rc::unwrap_or_clone(self.consumer),
        ) {
            (
                Term::Mu(Mu {
                    chi: Prd,
                    variable,
                    statement,
                }),
                Term::XVar(XVar { chi: Cns, var }),
            )
            | (
                Term::XVar(XVar { chi: Prd, var }),
                Term::Mu(Mu {
                    chi: Cns,
                    variable,
                    statement,
                }),
            ) => shrink_renaming(var, variable, statement, &self.ty, used_vars, types),

            (Term::Xtor(Xtor { id, args }), Term::XCase(XCase { clauses })) => {
                shrink_known_cuts(&id, args, clauses.as_slice(), used_vars, types)
            }

            (
                Term::XVar(XVar {
                    chi: Prd,
                    var: var_prd,
                }),
                Term::XVar(XVar {
                    chi: Cns,
                    var: var_cns,
                }),
            ) => shrink_unknown_cuts(var_prd, var_cns, self.ty, used_vars, types),

            (
                Term::Mu(Mu {
                    chi: Prd,
                    variable: var_prd,
                    statement: statement_prd,
                }),
                Term::Mu(Mu {
                    chi: Cns,
                    variable: var_cns,
                    statement: statement_cns,
                }),
            ) => shrink_critical_pairs(
                var_prd,
                statement_prd,
                var_cns,
                statement_cns,
                &self.ty,
                used_vars,
                types,
            ),

            (
                Term::Literal(Literal { lit }),
                Term::Mu(Mu {
                    chi: Cns,
                    variable,
                    statement,
                }),
            ) => shrink_literal_mu(lit, variable, statement, used_vars, types),

            (Term::Literal(Literal { lit }), Term::XVar(XVar { chi: Cns, var })) => {
                shrink_literal_var(lit, var, used_vars)
            }

            (
                Term::Xtor(Xtor { id, args }),
                Term::Mu(Mu {
                    chi: Cns,
                    variable,
                    statement,
                }),
            ) => axcut::syntax::Statement::Leta(axcut::syntax::statements::Leta {
                var: variable,
                ty: translate_ty(self.ty),
                tag: id,
                args,
                next: statement.shrink(used_vars, types),
            }),

            (Term::Xtor(Xtor { id, args }), Term::XVar(XVar { chi: Cns, var })) => {
                axcut::syntax::Statement::Invoke(axcut::syntax::statements::Invoke {
                    var,
                    tag: id,
                    ty: translate_ty(self.ty),
                    args,
                })
            }

            (Term::XVar(XVar { chi: Prd, var }), Term::XCase(XCase { clauses })) => {
                axcut::syntax::Statement::Switch(axcut::syntax::statements::Switch {
                    var,
                    ty: translate_ty(self.ty),
                    clauses: clauses.shrink(used_vars, types),
                })
            }

            (
                Term::Mu(Mu {
                    chi: Prd,
                    variable,
                    statement,
                }),
                Term::XCase(XCase { clauses }),
            ) => axcut::syntax::Statement::New(axcut::syntax::statements::New {
                var: variable,
                ty: translate_ty(self.ty),
                context: None,
                clauses: clauses.shrink(used_vars, types),
                next: statement.shrink(used_vars, types),
            }),

            _ => panic!("cannot happen"),
        }
    }
}

#[cfg(test)]
mod cut_tests {
    use super::Shrinking;
    use std::{collections::HashSet, rc::Rc};

    #[test]
    fn shrink_mu_var() {
        let result = core::syntax_var::statement::Cut {
            producer: Rc::new(
                core::syntax_var::term::Mu {
                    chi: core::syntax_var::Chirality::Prd,
                    variable: "a".to_owned(),
                    statement: Rc::new(core::syntax_var::Statement::Done()),
                }
                .into(),
            ),
            ty: core::syntax_var::types::Ty::Int,
            consumer: Rc::new(
                core::syntax_var::term::XVar {
                    chi: core::syntax_var::Chirality::Cns,
                    var: "x".to_owned(),
                }
                .into(),
            ),
        }
        .shrink(&mut HashSet::new(), &vec![]);
        let expected = axcut::syntax::statements::Return {
            var: "x".to_owned(),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn shrink_xtor_xcase() {
        let result = core::syntax_var::statement::Cut {
            producer: Rc::new(
                core::syntax_var::term::Xtor {
                    id: "Nil".to_owned(),
                    args: vec![],
                }
                .into(),
            ),
            ty: core::syntax_var::types::Ty::Decl("ListInt".to_owned()),
            consumer: Rc::new(
                core::syntax_var::term::XCase {
                    clauses: vec![
                        core::syntax_var::clause::Clause {
                            xtor: "Nil".to_owned(),
                            context: vec![],
                            case: Rc::new(core::syntax_var::Statement::Done()),
                        },
                        core::syntax_var::clause::Clause {
                            xtor: "Cons".to_owned(),
                            context: vec![
                                core::syntax_var::context::ContextBinding {
                                    var: "x".to_owned(),
                                    chi: core::syntax_var::Chirality::Prd,
                                    ty: core::syntax_var::types::Ty::Int,
                                },
                                core::syntax_var::context::ContextBinding {
                                    var: "xs".to_owned(),
                                    chi: core::syntax_var::Chirality::Prd,
                                    ty: core::syntax_var::types::Ty::Decl("ListInt".to_owned()),
                                },
                            ],
                            case: Rc::new(core::syntax_var::Statement::Done()),
                        },
                    ],
                }
                .into(),
            ),
        }
        .shrink(&mut HashSet::new(), &vec![]);
        let expected = axcut::syntax::Statement::Done;
        assert_eq!(result, expected)
    }

    #[test]
    fn shrink_var_var() {
        let result = core::syntax_var::statement::Cut {
            producer: Rc::new(core::syntax_var::term::XVar::var("x").into()),
            ty: core::syntax_var::types::Ty::Int,
            consumer: Rc::new(core::syntax_var::term::XVar::covar("a").into()),
        }
        .shrink(&mut HashSet::new(), &vec![]);
        let int_ty = core::syntax_var::declaration::cont_int();
        let expected = axcut::syntax::statements::Invoke {
            var: "a".to_owned(),
            tag: int_ty.xtors[0].name.clone(),
            ty: axcut::syntax::Ty::Decl(int_ty.name),
            args: vec!["x".to_owned()],
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn shrink_mu_mu() {
        let result = core::syntax_var::statement::Cut {
            producer: Rc::new(
                core::syntax_var::term::Mu::mu("a", core::syntax_var::Statement::Done()).into(),
            ),

            ty: core::syntax_var::types::Ty::Int,
            consumer: Rc::new(
                core::syntax_var::term::Mu::tilde_mu("x", core::syntax_var::Statement::Done())
                    .into(),
            ),
        }
        .shrink(&mut HashSet::new(), &vec![]);
        let int_ty = core::syntax_var::declaration::cont_int();
        let expected = axcut::syntax::statements::New {
            var: "a".to_owned(),
            ty: axcut::syntax::Ty::Decl(int_ty.name),
            context: None,
            clauses: vec![axcut::syntax::Clause {
                xtor: int_ty.xtors[0].name.clone(),
                context: vec![axcut::syntax::ContextBinding {
                    var: "x".to_owned(),
                    chi: axcut::syntax::Chirality::Ext,
                    ty: axcut::syntax::Ty::Int,
                }],
                case: Rc::new(
                    axcut::syntax::statements::Return {
                        var: "x".to_owned(),
                    }
                    .into(),
                ),
            }],
            next: Rc::new(axcut::syntax::Statement::Done),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn shrink_lit_mu() {
        let result = core::syntax_var::statement::Cut {
            producer: Rc::new(core::syntax_var::term::Literal { lit: 1 }.into()),
            ty: core::syntax_var::types::Ty::Int,
            consumer: Rc::new(
                core::syntax_var::term::Mu::tilde_mu("x", core::syntax_var::Statement::Done())
                    .into(),
            ),
        }
        .shrink(&mut HashSet::new(), &vec![]);
        let expected = axcut::syntax::statements::Literal {
            lit: 1,
            var: "x".to_owned(),
            case: Rc::new(
                axcut::syntax::statements::Return {
                    var: "x".to_owned(),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn shrink_lit_var() {
        let result = core::syntax_var::statement::Cut {
            producer: Rc::new(core::syntax_var::term::Literal { lit: 1 }.into()),
            ty: core::syntax_var::types::Ty::Int,
            consumer: Rc::new(core::syntax_var::term::XVar::covar("a").into()),
        }
        .shrink(&mut HashSet::new(), &vec![]);
        let int_ty = core::syntax_var::declaration::cont_int();
        let expected = axcut::syntax::statements::Literal {
            lit: 1,
            var: "x0".to_owned(),
            case: Rc::new(axcut::syntax::Statement::Invoke(
                axcut::syntax::statements::Invoke {
                    var: "a".to_owned(),
                    tag: int_ty.xtors[0].name.clone(),
                    ty: axcut::syntax::Ty::Decl(int_ty.name),
                    args: vec!["x0".to_owned()],
                },
            )),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn shrink_xtor_mu() {
        let result = core::syntax_var::statement::Cut {
            producer: Rc::new(
                core::syntax_var::term::Xtor {
                    id: "Nil".to_owned(),
                    args: vec![],
                }
                .into(),
            ),
            ty: core::syntax_var::types::Ty::Decl("ListInt".to_owned()),
            consumer: Rc::new(
                core::syntax_var::term::Mu::tilde_mu("x", core::syntax_var::Statement::Done())
                    .into(),
            ),
        }
        .shrink(&mut HashSet::new(), &vec![]);
        let expected = axcut::syntax::statements::Leta {
            var: "x".to_owned(),
            ty: axcut::syntax::types::Ty::Decl("ListInt".to_owned()),
            tag: "Nil".to_owned(),
            args: vec![],
            next: Rc::new(axcut::syntax::Statement::Done),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn shrink_xtor_var() {
        let result = core::syntax_var::statement::Cut {
            producer: Rc::new(
                core::syntax_var::term::Xtor {
                    id: "Nil".to_owned(),
                    args: vec![],
                }
                .into(),
            ),
            ty: core::syntax_var::types::Ty::Decl("ListInt".to_owned()),
            consumer: Rc::new(core::syntax_var::term::XVar::covar("a").into()),
        }
        .shrink(&mut HashSet::new(), &vec![]);
        let expected = axcut::syntax::statements::Invoke {
            var: "a".to_owned(),
            tag: "Nil".to_owned(),
            ty: axcut::syntax::types::Ty::Decl("ListInt".to_owned()),
            args: vec![],
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn shrink_var_case() {
        let result = core::syntax_var::statement::Cut {
            producer: Rc::new(core::syntax_var::term::XVar::var("x").into()),
            ty: core::syntax_var::types::Ty::Decl("ListInt".to_owned()),
            consumer: Rc::new(
                core::syntax_var::term::XCase {
                    clauses: vec![
                        core::syntax_var::clause::Clause {
                            xtor: "Nil".to_owned(),
                            context: vec![],
                            case: Rc::new(core::syntax_var::Statement::Done()),
                        },
                        core::syntax_var::clause::Clause {
                            xtor: "Cons".to_owned(),
                            context: vec![
                                core::syntax_var::context::ContextBinding {
                                    var: "x".to_owned(),
                                    chi: core::syntax_var::Chirality::Prd,
                                    ty: core::syntax_var::types::Ty::Int,
                                },
                                core::syntax_var::context::ContextBinding {
                                    var: "xs".to_owned(),
                                    chi: core::syntax_var::Chirality::Prd,
                                    ty: core::syntax_var::types::Ty::Decl("ListInt".to_owned()),
                                },
                            ],
                            case: Rc::new(core::syntax_var::Statement::Done()),
                        },
                    ],
                }
                .into(),
            ),
        }
        .shrink(&mut HashSet::new(), &vec![]);
        let expected = axcut::syntax::statements::Switch {
            var: "x".to_owned(),
            ty: axcut::syntax::types::Ty::Decl("ListInt".to_owned()),
            clauses: vec![
                axcut::syntax::clause::Clause {
                    xtor: "Nil".to_owned(),
                    context: vec![],
                    case: Rc::new(axcut::syntax::Statement::Done),
                },
                axcut::syntax::clause::Clause {
                    xtor: "Cons".to_owned(),
                    context: vec![
                        axcut::syntax::context::ContextBinding {
                            var: "x".to_owned(),
                            chi: axcut::syntax::Chirality::Ext,
                            ty: axcut::syntax::types::Ty::Int,
                        },
                        axcut::syntax::context::ContextBinding {
                            var: "xs".to_owned(),
                            chi: axcut::syntax::Chirality::Prd,
                            ty: axcut::syntax::types::Ty::Decl("ListInt".to_owned()),
                        },
                    ],
                    case: Rc::new(axcut::syntax::Statement::Done),
                },
            ],
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn shrink_mu_case() {
        let result = core::syntax_var::statement::Cut {
            producer: Rc::new(
                core::syntax_var::term::Mu::mu("a", core::syntax_var::Statement::Done()).into(),
            ),
            ty: core::syntax_var::types::Ty::Decl("ListInt".to_owned()),
            consumer: Rc::new(
                core::syntax_var::term::XCase {
                    clauses: vec![
                        core::syntax_var::clause::Clause {
                            xtor: "Nil".to_owned(),
                            context: vec![],
                            case: Rc::new(core::syntax_var::Statement::Done()),
                        },
                        core::syntax_var::clause::Clause {
                            xtor: "Cons".to_owned(),
                            context: vec![
                                core::syntax_var::context::ContextBinding {
                                    var: "x".to_owned(),
                                    chi: core::syntax_var::Chirality::Prd,
                                    ty: core::syntax_var::types::Ty::Int,
                                },
                                core::syntax_var::context::ContextBinding {
                                    var: "xs".to_owned(),
                                    chi: core::syntax_var::Chirality::Prd,
                                    ty: core::syntax_var::types::Ty::Decl("ListInt".to_owned()),
                                },
                            ],
                            case: Rc::new(core::syntax_var::Statement::Done()),
                        },
                    ],
                }
                .into(),
            ),
        }
        .shrink(&mut HashSet::new(), &vec![]);
        let expected = axcut::syntax::statements::New {
            var: "a".to_owned(),
            ty: axcut::syntax::types::Ty::Decl("ListInt".to_owned()),
            context: None,
            clauses: vec![
                axcut::syntax::clause::Clause {
                    xtor: "Nil".to_owned(),
                    context: vec![],
                    case: Rc::new(axcut::syntax::Statement::Done),
                },
                axcut::syntax::clause::Clause {
                    xtor: "Cons".to_owned(),
                    context: vec![
                        axcut::syntax::context::ContextBinding {
                            var: "x".to_owned(),
                            chi: axcut::syntax::Chirality::Ext,
                            ty: axcut::syntax::types::Ty::Int,
                        },
                        axcut::syntax::context::ContextBinding {
                            var: "xs".to_owned(),
                            chi: axcut::syntax::Chirality::Prd,
                            ty: axcut::syntax::types::Ty::Decl("ListInt".to_owned()),
                        },
                    ],
                    case: Rc::new(axcut::syntax::Statement::Done),
                },
            ],
            next: Rc::new(axcut::syntax::Statement::Done),
        }
        .into();
        assert_eq!(result, expected)
    }
}
