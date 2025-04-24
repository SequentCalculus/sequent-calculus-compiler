use core_lang::syntax::{
    Name, Ty, TypingContext, Var,
    declaration::{cont_int, lookup_type_declaration},
    fresh_name, fresh_var,
    statements::{FsCut, FsStatement},
    terms::*,
};
use core_lang::traits::*;

use crate::context::shrink_context;
use crate::shrinking::{Shrinking, ShrinkingState};
use crate::types::shrink_ty;

use std::collections::{BTreeSet, HashSet};
use std::rc::Rc;

fn shrink_renaming(
    var: Var,
    var_mu: Var,
    statement: Rc<FsStatement>,
    ty: &Ty,
    state: &mut ShrinkingState,
) -> axcut::syntax::Statement {
    if *ty == Ty::I64 && *statement == FsStatement::Done() {
        axcut::syntax::statements::Return { var }.into()
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
    let (statement, context) = match clauses.iter().find(|clause| clause.xtor == *id) {
        None => panic!("Xtor {id} not found in clauses {clauses:?}"),
        Some(clause) => (clause.body.clone(), &clause.context),
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
        Ty::I64 => axcut::syntax::statements::Invoke {
            var: var_cns,
            tag: cont_int().xtors[0].name.clone(),
            ty: axcut::syntax::Ty::Decl(cont_int().name),
            args: vec![var_prd],
        }
        .into(),

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

            let translated_ty = shrink_ty(ty);

            let clauses: Vec<axcut::syntax::statements::Clause> = xtors
                .into_iter()
                .map(|(xtor, args)| {
                    let env: axcut::syntax::TypingContext = shrink_context(args, state.codata)
                        .bindings
                        .into_iter()
                        .map(|binding| axcut::syntax::ContextBinding {
                            var: fresh_name(state.used_vars, &binding.var),
                            ..binding
                        })
                        .collect::<Vec<_>>()
                        .into();
                    axcut::syntax::statements::Clause {
                        xtor: xtor.clone(),
                        context: env.clone(),
                        body: Rc::new(
                            axcut::syntax::statements::Invoke {
                                var: var_expand.clone(),
                                tag: xtor,
                                ty: translated_ty.clone(),
                                args: env.vars(),
                            }
                            .into(),
                        ),
                    }
                })
                .collect();

            axcut::syntax::statements::Switch {
                var: var_keep,
                ty: translated_ty,
                clauses,
                free_vars_clauses: None,
            }
            .into()
        }
    }
}

fn lift(statement: FsStatement, state: &mut ShrinkingState) -> Rc<axcut::syntax::Statement> {
    // the free variables of the statement ...
    let mut typed_free_vars = BTreeSet::new();
    statement.typed_free_vars(&mut typed_free_vars);
    // ... become the signature of the lifted label ...
    let context = shrink_context(
        TypingContext {
            bindings: typed_free_vars.into_iter().collect(),
        },
        state.codata,
    );
    // ... and the arguments of the call to it
    let args = context.vars();

    let label = fresh_name(
        state.used_labels,
        &("lift_".to_string() + state.current_label + "_"),
    );
    let body = statement.shrink(state);

    state.lifted_statements.push_front(axcut::syntax::Def {
        name: label.clone(),
        context,
        body,
        used_vars: state.used_vars.clone(),
    });

    Rc::new(axcut::syntax::statements::Call { label, args }.into())
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
            let body = if *statement_cns == FsStatement::Done() {
                Rc::new(
                    axcut::syntax::statements::Return {
                        var: var_cns.clone(),
                    }
                    .into(),
                )
            } else {
                statement_cns.shrink(state)
            };

            axcut::syntax::statements::New {
                var: var_prd,
                ty: axcut::syntax::Ty::Decl(cont_int().name),
                context: None,
                clauses: vec![axcut::syntax::statements::Clause {
                    xtor: cont_int().xtors[0].name.clone(),
                    context: vec![axcut::syntax::ContextBinding {
                        var: var_cns,
                        chi: axcut::syntax::Chirality::Ext,
                        ty: axcut::syntax::Ty::I64,
                    }]
                    .into(),
                    body,
                }],
                free_vars_clauses: None,
                next: statement_prd.shrink(state),
                free_vars_next: None,
            }
            .into()
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

            let translated_ty = shrink_ty(ty);

            // if there is more than one clause and the statement of the expanded side is a
            // non-leaf statement, we share it by lifting it to the top level to avoid
            // exponential blowup
            let shrunk_statement_expand = if xtors.len() <= 1
                || matches!(
                    *statement_expand,
                    FsStatement::Done() | FsStatement::Call(_)
                )
                // check if the statement will become an `invoke` statement
                || matches!(&*statement_expand, FsStatement::Cut(FsCut { producer, consumer, .. })
                    if (matches!(**producer, FsTerm::XVar(_)) && matches!(**consumer, FsTerm::Xtor(_)))
                    || (matches!(**producer, FsTerm::Xtor(_)) && matches!(**consumer, FsTerm::XVar(_)))
                ) {
                statement_expand.shrink(state)
            } else {
                lift(Rc::unwrap_or_clone(statement_expand), state)
            };

            let clauses: Vec<axcut::syntax::statements::Clause> = xtors
                .into_iter()
                .map(|(xtor, args)| {
                    let env: axcut::syntax::TypingContext = shrink_context(args, state.codata)
                        .bindings
                        .into_iter()
                        .map(|binding| axcut::syntax::ContextBinding {
                            var: fresh_name(state.used_vars, &binding.var),
                            ..binding
                        })
                        .collect::<Vec<_>>()
                        .into();
                    axcut::syntax::statements::Clause {
                        xtor: xtor.clone(),
                        context: env.clone(),
                        body: Rc::new(
                            axcut::syntax::statements::Let {
                                var: var_expand.clone(),
                                ty: translated_ty.clone(),
                                tag: xtor,
                                args: env.vars(),
                                next: shrunk_statement_expand.clone(),
                                free_vars_next: None,
                            }
                            .into(),
                        ),
                    }
                })
                .collect();

            axcut::syntax::statements::New {
                var: var_keep,
                ty: axcut::syntax::Ty::Decl(name),
                context: None,
                clauses,
                free_vars_clauses: None,
                next: statement_keep.shrink(state),
                free_vars_next: None,
            }
            .into()
        }
    }
}

fn shrink_literal_mu(
    lit: i64,
    var: Var,
    statement: Rc<FsStatement>,
    state: &mut ShrinkingState,
) -> axcut::syntax::Statement {
    let next = if *statement == FsStatement::Done() {
        Rc::new(axcut::syntax::statements::Return { var: var.clone() }.into())
    } else {
        statement.shrink(state)
    };

    axcut::syntax::statements::Literal {
        lit,
        var,
        next,
        free_vars_next: None,
    }
    .into()
}

fn shrink_literal_var(
    lit: i64,
    var: Var,
    used_vars: &mut HashSet<Var>,
) -> axcut::syntax::Statement {
    let fresh_var = fresh_var(used_vars);
    axcut::syntax::statements::Literal {
        lit,
        var: fresh_var.clone(),
        next: Rc::new(
            axcut::syntax::statements::Invoke {
                var,
                tag: cont_int().xtors[0].name.clone(),
                ty: axcut::syntax::Ty::Decl(cont_int().name),
                args: vec![fresh_var],
            }
            .into(),
        ),
        free_vars_next: None,
    }
    .into()
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
                    ty: _,
                }),
                FsTerm::XCase(XCase {
                    prdcns: Cns,
                    clauses,
                    ..
                }),
            ) => shrink_known_cuts(&id, args.vec_vars(), clauses.as_slice(), state),
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
                    ty: _,
                }),
            ) => shrink_known_cuts(&id, args.vec_vars(), clauses.as_slice(), state),

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
                    ty: _,
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
                    ty: _,
                }),
            ) => axcut::syntax::statements::Let {
                var: variable,
                ty: shrink_ty(self.ty),
                tag: id,
                args: args.vec_vars(),
                next: statement.shrink(state),
                free_vars_next: None,
            }
            .into(),

            (
                FsTerm::Xtor(FsXtor {
                    prdcns: Prd,
                    id,
                    args,
                    ty: _,
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
                    ty: _,
                }),
            ) => axcut::syntax::statements::Invoke {
                var,
                tag: id,
                ty: shrink_ty(self.ty),
                args: args.vec_vars(),
            }
            .into(),

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
            ) => axcut::syntax::statements::Switch {
                var,
                ty: shrink_ty(self.ty),
                clauses: clauses.shrink(state),
                free_vars_clauses: None,
            }
            .into(),
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
            ) => axcut::syntax::statements::Switch {
                var,
                ty: shrink_ty(self.ty),
                clauses: clauses.shrink(state),
                free_vars_clauses: None,
            }
            .into(),

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
            ) => axcut::syntax::statements::New {
                var: variable,
                ty: shrink_ty(self.ty),
                context: None,
                clauses: clauses.shrink(state),
                free_vars_clauses: None,
                next: statement.shrink(state),
                free_vars_next: None,
            }
            .into(),
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
            ) => axcut::syntax::statements::New {
                var: variable,
                ty: shrink_ty(self.ty),
                context: None,
                clauses: clauses.shrink(state),
                free_vars_clauses: None,
                next: statement.shrink(state),
                free_vars_next: None,
            }
            .into(),

            _ => panic!("cannot happen"),
        }
    }
}
