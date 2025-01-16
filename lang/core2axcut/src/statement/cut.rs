use core_lang::syntax::{
    declaration::{cont_int, lookup_type_declaration},
    fresh_name, fresh_var,
    statement::{FsCut, FsStatement},
    term::*,
    Name, Ty, Var,
};
use core_lang::traits::*;

use crate::context::translate_context;
use crate::traits::{Shrinking, ShrinkingState};
use crate::types::translate_ty;

use std::{collections::HashSet, rc::Rc};

fn shrink_renaming(
    var: Var,
    var_mu: Var,
    statement: Rc<FsStatement>,
    ty: &Ty,
    state: &mut ShrinkingState,
) -> axcut::syntax::Statement {
    if *ty == Ty::I64 && *statement == FsStatement::Done() {
        axcut::syntax::Statement::Return(axcut::syntax::statements::Return { var })
    } else {
        Rc::unwrap_or_clone(statement)
            .subst_sim(&[(var_mu, var)])
            .shrink(state)
    }
}

fn shrink_known_cuts<T: PrdCns + std::fmt::Debug>(
    id: &Name,
    args: Vec<Var>,
    clauses: &[Clause<T, FsStatement>],
    state: &mut ShrinkingState,
) -> axcut::syntax::Statement {
    let (statement, context) = match clauses.iter().find(
        |Clause {
             xtor,
             context: _,
             rhs: _,
             prdcns: _,
         }| xtor == id,
    ) {
        None => panic!("Xtor {id} not found in clauses {clauses:?}"),
        Some(Clause {
            xtor: _,
            context,
            rhs,
            prdcns: _,
        }) => (rhs.clone(), context),
    };
    let subst: Vec<(Var, Var)> = context.vec_vars().into_iter().zip(args).collect();
    Rc::unwrap_or_clone(statement)
        .subst_sim(subst.as_slice())
        .shrink(state)
}

fn shrink_unknown_cuts(
    var_prd: Var,
    var_cns: Var,
    ty: Ty,
    state: &mut ShrinkingState,
) -> axcut::syntax::Statement {
    match ty.clone() {
        Ty::I64 => axcut::syntax::Statement::Invoke(axcut::syntax::statements::Invoke {
            var: var_cns,
            tag: cont_int().xtors[0].name.clone(),
            ty: axcut::syntax::Ty::Decl(cont_int().name),
            args: vec![var_prd],
        }),
        Ty::Decl(name) => {
            let (xtors, var_keep, var_expand): (Vec<_>, _, _) = if ty.is_codata(state.codata) {
                (
                    lookup_type_declaration(&name, state.codata)
                        .xtors
                        .iter()
                        .map(|xtor| (xtor.name.clone(), xtor.args.clone()))
                        .collect(),
                    var_cns,
                    var_prd,
                )
            } else {
                (
                    lookup_type_declaration(&name, state.data)
                        .xtors
                        .iter()
                        .map(|xtor| (xtor.name.clone(), xtor.args.clone()))
                        .collect(),
                    var_prd,
                    var_cns,
                )
            };
            let translated_ty = translate_ty(ty);
            let clauses: Vec<axcut::syntax::Clause> = xtors
                .into_iter()
                .map(|(xtor, args)| {
                    let env: axcut::syntax::TypingContext = translate_context(args, state.codata)
                        .bindings
                        .into_iter()
                        .map(|binding| axcut::syntax::ContextBinding {
                            var: fresh_name(state.used_vars, &binding.var),
                            ..binding
                        })
                        .collect::<Vec<_>>()
                        .into();
                    axcut::syntax::Clause {
                        xtor: xtor.clone(),
                        context: env.clone(),
                        case: Rc::new(axcut::syntax::Statement::Invoke(
                            axcut::syntax::statements::Invoke {
                                var: var_expand.clone(),
                                tag: xtor,
                                ty: translated_ty.clone(),
                                args: env.vars(),
                            },
                        )),
                    }
                })
                .collect();
            axcut::syntax::Statement::Switch(axcut::syntax::statements::Switch {
                var: var_keep,
                ty: translated_ty,
                clauses,
            })
        }
    }
}

fn shrink_critical_pairs(
    var_prd: Var,
    statement_prd: Rc<FsStatement>,
    var_cns: Var,
    statement_cns: Rc<FsStatement>,
    ty: Ty,
    state: &mut ShrinkingState,
) -> axcut::syntax::Statement {
    match ty.clone() {
        Ty::I64 => {
            let case = if *statement_cns == FsStatement::Done() {
                Rc::new(axcut::syntax::Statement::Return(
                    axcut::syntax::statements::Return {
                        var: var_cns.clone(),
                    },
                ))
            } else {
                statement_cns.shrink(state)
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
                        ty: axcut::syntax::Ty::I64,
                    }]
                    .into(),
                    case,
                }],
                next: statement_prd.shrink(state),
            })
        }
        Ty::Decl(name) => {
            let (xtors, var_keep, statement_keep, var_expand, statement_expand): (
                Vec<_>,
                _,
                _,
                _,
                _,
            ) = if ty.is_codata(state.codata) {
                (
                    lookup_type_declaration(&name, state.codata)
                        .xtors
                        .iter()
                        .map(|xtor| (xtor.name.clone(), xtor.args.clone()))
                        .collect(),
                    var_cns,
                    statement_cns,
                    var_prd,
                    statement_prd,
                )
            } else {
                (
                    lookup_type_declaration(&name, state.data)
                        .xtors
                        .iter()
                        .map(|xtor| (xtor.name.clone(), xtor.args.clone()))
                        .collect(),
                    var_prd,
                    statement_prd,
                    var_cns,
                    statement_cns,
                )
            };
            let translated_ty = translate_ty(ty);
            let shrunk_statement_expand = statement_expand.shrink(state);
            let clauses: Vec<axcut::syntax::Clause> = xtors
                .into_iter()
                .map(|(xtor, args)| {
                    let env: axcut::syntax::TypingContext = translate_context(args, state.codata)
                        .bindings
                        .into_iter()
                        .map(|binding| axcut::syntax::ContextBinding {
                            var: fresh_name(state.used_vars, &binding.var),
                            ..binding
                        })
                        .collect::<Vec<_>>()
                        .into();
                    axcut::syntax::Clause {
                        xtor: xtor.clone(),
                        context: env.clone(),
                        case: Rc::new(axcut::syntax::Statement::Leta(
                            axcut::syntax::statements::Leta {
                                var: var_expand.clone(),
                                ty: translated_ty.clone(),
                                tag: xtor,
                                args: env.vars(),
                                next: shrunk_statement_expand.clone(),
                            },
                        )),
                    }
                })
                .collect();
            axcut::syntax::Statement::New(axcut::syntax::statements::New {
                var: var_keep,
                ty: axcut::syntax::Ty::Decl(name),
                context: None,
                clauses,
                next: statement_keep.shrink(state),
            })
        }
    }
}

fn shrink_literal_mu(
    lit: i64,
    var: Var,
    statement: Rc<FsStatement>,
    state: &mut ShrinkingState,
) -> axcut::syntax::Statement {
    let case = if *statement == FsStatement::Done() {
        Rc::new(axcut::syntax::Statement::Return(
            axcut::syntax::statements::Return { var: var.clone() },
        ))
    } else {
        statement.shrink(state)
    };
    axcut::syntax::Statement::Literal(axcut::syntax::statements::Literal { lit, var, case })
}

fn shrink_literal_var(
    lit: i64,
    var: Var,
    used_vars: &mut HashSet<Var>,
) -> axcut::syntax::Statement {
    let fresh_var = fresh_var(used_vars);
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

impl Shrinking for FsCut {
    type Target = axcut::syntax::Statement;

    fn shrink(self, state: &mut ShrinkingState) -> axcut::syntax::Statement {
        match (
            Rc::unwrap_or_clone(self.producer),
            Rc::unwrap_or_clone(self.consumer),
        ) {
            (
                FsTerm::Mu(Mu {
                    prdcns: Prd,
                    variable,
                    statement,
                    ..
                }),
                FsTerm::XVar(XVar {
                    prdcns: Cns,
                    var,
                    ty: _,
                }),
            )
            | (
                FsTerm::XVar(XVar {
                    prdcns: Prd,
                    var,
                    ty: _,
                }),
                FsTerm::Mu(Mu {
                    prdcns: Cns,
                    variable,
                    statement,
                    ..
                }),
            ) => shrink_renaming(var, variable, statement, &self.ty, state),

            (
                FsTerm::Xtor(FsXtor {
                    prdcns: Prd,
                    id,
                    args,
                }),
                FsTerm::XCase(XCase {
                    prdcns: Cns,
                    clauses,
                    ..
                }),
            ) => shrink_known_cuts(&id, args, clauses.as_slice(), state),
            (
                FsTerm::XCase(XCase {
                    prdcns: Prd,
                    clauses,
                    ..
                }),
                FsTerm::Xtor(FsXtor {
                    prdcns: Cns,
                    id,
                    args,
                }),
            ) => shrink_known_cuts(&id, args, clauses.as_slice(), state),

            (
                FsTerm::XVar(XVar {
                    prdcns: Prd,
                    var: var_prd,
                    ty: _,
                }),
                FsTerm::XVar(XVar {
                    prdcns: Cns,
                    var: var_cns,
                    ty: _,
                }),
            ) => shrink_unknown_cuts(var_prd, var_cns, self.ty, state),

            (
                FsTerm::Mu(Mu {
                    prdcns: Prd,
                    variable: var_prd,
                    statement: statement_prd,
                    ..
                }),
                FsTerm::Mu(Mu {
                    prdcns: Cns,
                    variable: var_cns,
                    statement: statement_cns,
                    ..
                }),
            ) => shrink_critical_pairs(
                var_prd,
                statement_prd,
                var_cns,
                statement_cns,
                self.ty,
                state,
            ),

            (
                FsTerm::Literal(Literal { lit }),
                FsTerm::Mu(Mu {
                    prdcns: Cns,
                    variable,
                    statement,
                    ..
                }),
            ) => shrink_literal_mu(lit, variable, statement, state),

            (
                FsTerm::Literal(Literal { lit }),
                FsTerm::XVar(XVar {
                    prdcns: Cns,
                    var,
                    ty: _,
                }),
            ) => shrink_literal_var(lit, var, state.used_vars),

            (
                FsTerm::Xtor(FsXtor {
                    prdcns: Prd,
                    id,
                    args,
                }),
                FsTerm::Mu(Mu {
                    prdcns: Cns,
                    variable,
                    statement,
                    ..
                }),
            )
            | (
                FsTerm::Mu(Mu {
                    prdcns: Prd,
                    variable,
                    statement,
                    ..
                }),
                FsTerm::Xtor(FsXtor {
                    prdcns: Cns,
                    id,
                    args,
                }),
            ) => axcut::syntax::Statement::Leta(axcut::syntax::statements::Leta {
                var: variable,
                ty: translate_ty(self.ty),
                tag: id,
                args,
                next: statement.shrink(state),
            }),

            (
                FsTerm::Xtor(FsXtor {
                    prdcns: Prd,
                    id,
                    args,
                }),
                FsTerm::XVar(XVar {
                    prdcns: Cns,
                    var,
                    ty: _,
                }),
            )
            | (
                FsTerm::XVar(XVar {
                    prdcns: Prd,
                    var,
                    ty: _,
                }),
                FsTerm::Xtor(FsXtor {
                    prdcns: Cns,
                    id,
                    args,
                }),
            ) => axcut::syntax::Statement::Invoke(axcut::syntax::statements::Invoke {
                var,
                tag: id,
                ty: translate_ty(self.ty),
                args,
            }),

            (
                FsTerm::XVar(XVar {
                    prdcns: Prd,
                    var,
                    ty: _,
                }),
                FsTerm::XCase(XCase {
                    prdcns: Cns,
                    clauses,
                    ..
                }),
            ) => axcut::syntax::Statement::Switch(axcut::syntax::statements::Switch {
                var,
                ty: translate_ty(self.ty),
                clauses: clauses.shrink(state),
            }),
            (
                FsTerm::XCase(XCase {
                    prdcns: Prd,
                    clauses,
                    ..
                }),
                FsTerm::XVar(XVar {
                    prdcns: Cns,
                    var,
                    ty: _,
                }),
            ) => axcut::syntax::Statement::Switch(axcut::syntax::statements::Switch {
                var,
                ty: translate_ty(self.ty),
                clauses: clauses.shrink(state),
            }),

            (
                FsTerm::Mu(Mu {
                    prdcns: Prd,
                    variable,
                    statement,
                    ..
                }),
                FsTerm::XCase(XCase {
                    prdcns: Cns,
                    clauses,
                    ..
                }),
            ) => axcut::syntax::Statement::New(axcut::syntax::statements::New {
                var: variable,
                ty: translate_ty(self.ty),
                context: None,
                clauses: clauses.shrink(state),
                next: statement.shrink(state),
            }),
            (
                FsTerm::XCase(XCase {
                    prdcns: Prd,
                    clauses,
                    ..
                }),
                FsTerm::Mu(Mu {
                    prdcns: Cns,
                    variable,
                    statement,
                    ..
                }),
            ) => axcut::syntax::Statement::New(axcut::syntax::statements::New {
                var: variable,
                ty: translate_ty(self.ty),
                context: None,
                clauses: clauses.shrink(state),
                next: statement.shrink(state),
            }),

            _ => panic!("cannot happen"),
        }
    }
}
