//! This module defines the translation of cuts.

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

/// This function function eliminates the cut of a (co)variable and a (tilde-)mu-binding by
/// reduction.
/// - `var` is the (co)variable on one side of the cut.
/// - `var_mu` is the variable bound by the (tilde-)mu.
/// - `statement` is the body of the (tilde-)mu.
/// - `state` is the state of the whole translation.
fn shrink_renaming(
    var: Var,
    var_mu: Var,
    statement: Rc<FsStatement>,
    state: &mut ShrinkingState,
) -> axcut::syntax::Statement {
    Rc::unwrap_or_clone(statement)
        .subst_sim(&[(var_mu, var)])
        .shrink(state)
}

/// This function function eliminates the cut of an xtor and a (co)match by reduction.
/// - `id` is the name of the xtor.
/// - `args` is the argument list of the xtor.
/// - `clauses` are the clauses of the (co)match.
/// - `state` is the state of the whole translation.
///
/// # Panics
///
/// A panic is caused if no clause for the xtor is in the (co)match.
fn shrink_known_cuts<T: Chi + std::fmt::Debug>(
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

/// This function function eliminates the cut of a variable and a covariable.
/// - `var_prd` is the variable.
/// - `var_cns` is the covariable.
/// - `ty` is the type of the variable and the covariable.
/// - `state` is the state of the whole translation.
fn shrink_unknown_cuts(
    var_prd: Var,
    var_cns: Var,
    ty: Ty,
    state: &mut ShrinkingState,
) -> axcut::syntax::Statement {
    match ty.clone() {
        // for integers the type of the covariable becomes the continuation type ...
        Ty::I64 => axcut::syntax::statements::Invoke {
            var: var_cns,
            // ... so we wrap the variable into a continuation xtor
            tag: cont_int().xtors[0].name.clone(),
            ty: axcut::syntax::Ty::Decl(cont_int().name),
            args: vec![axcut::syntax::ContextBinding {
                var: var_prd,
                chi: axcut::syntax::Chirality::Ext,
                ty: axcut::syntax::Ty::I64,
            }]
            .into(),
        }
        .into(),

        // otherwise we eta-expand one side, depending on whether the type is a data or codata type
        Ty::Decl(name) => {
            // for codata types we flip the sides of the cut, then we can always expand the
            // right-hand side
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

            // we generate clauses binding fresh variables for the eta-expansion according to the
            // type
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
                            // we invoke the method of each clause on the expanded (co)variable
                            axcut::syntax::statements::Invoke {
                                var: var_expand.clone(),
                                tag: xtor,
                                ty: translated_ty.clone(),
                                args: env,
                            }
                            .into(),
                        ),
                    }
                })
                .collect();

            // we match on the unexpanded (co)variable
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

/// This functions lifts a statement to the top-level for sharing.
/// - `statement` is the statement to lift.
/// - `state` is the state of the whole translation.
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
    let args = context.clone();

    let label = fresh_name(
        state.used_labels,
        &("lift_".to_string() + state.current_label + "_"),
    );
    let body = statement.shrink(state);

    // we collect all lifted statements for the current top-level function
    state.lifted_statements.push_front(axcut::syntax::Def {
        name: label.clone(),
        context: context.clone(),
        body,
        used_vars: state.used_vars.clone(),
    });

    // ... and the arguments of the call to it
    Rc::new(axcut::syntax::statements::Call { label, args }.into())
}

/// This function function eliminates the cut of a mu- and a tilde-mu-binding, i.e., critical
/// pairs.
/// - `var_prd` is the covariable bound by the mu-binding.
/// - `statement_prd` is the body of the mu-binding.
/// - `var_cns` is the variable bound by the tilde-mu-binding.
/// - `statement_cns` is the body of the tilde-mu-binding.
/// - `ty` is the type of the mu- and the tilde-mu-binding.
/// - `state` is the state of the whole translation.
fn shrink_critical_pairs(
    var_prd: Var,
    statement_prd: Rc<FsStatement>,
    var_cns: Var,
    statement_cns: Rc<FsStatement>,
    ty: Ty,
    state: &mut ShrinkingState,
) -> axcut::syntax::Statement {
    match ty.clone() {
        // for integers the type of the bound covariable becomes the continuation type ...
        Ty::I64 => axcut::syntax::statements::Create {
            var: var_prd,
            ty: axcut::syntax::Ty::Decl(cont_int().name),
            // ... so we turn the tilde-mu-binding into a continuation clsoure
            context: None,
            clauses: vec![axcut::syntax::statements::Clause {
                xtor: cont_int().xtors[0].name.clone(),
                context: vec![axcut::syntax::ContextBinding {
                    var: var_cns,
                    chi: axcut::syntax::Chirality::Ext,
                    ty: axcut::syntax::Ty::I64,
                }]
                .into(),
                body: statement_cns.shrink(state),
            }],
            free_vars_clauses: None,
            next: statement_prd.shrink(state),
            free_vars_next: None,
        }
        .into(),

        // otherwise we eta-expand one side, depending on whether the type is a data or codata type
        Ty::Decl(name) => {
            // for codata types we flip the sides of the cut, then we can always expand the
            // right-hand side
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
            // non-leaf statement, we share it by lifting it to the top level to avoid exponential
            // blowup
            let shrunk_statement_expand = if xtors.len() <= 1
                || matches!(
                    *statement_expand,
                    FsStatement::Exit(_) | FsStatement::Call(_)
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

            // we generate clauses binding fresh variables for the eta-expansion according to the
            // type
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
                            // we bind the xtor of each clause with the expanded binding
                            axcut::syntax::statements::Let {
                                var: var_expand.clone(),
                                ty: translated_ty.clone(),
                                tag: xtor,
                                args: env,
                                next: shrunk_statement_expand.clone(),
                                free_vars_next: None,
                            }
                            .into(),
                        ),
                    }
                })
                .collect();

            // we bind the created closure with the unexpanded binding
            axcut::syntax::statements::Create {
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

/// This function function eliminates the cut of an integer literal and a tilde-mu-binding.
/// - `lit` is the integer literal.
/// - `var` is the variable bound by the tilde-mu.
/// - `statement` is the body of the tilde-mu.
/// - `state` is the state of the whole translation.
fn shrink_literal_mu(
    lit: i64,
    var: Var,
    statement: Rc<FsStatement>,
    state: &mut ShrinkingState,
) -> axcut::syntax::Statement {
    axcut::syntax::statements::Literal {
        lit,
        var,
        next: statement.shrink(state),
        free_vars_next: None,
    }
    .into()
}

/// This function function eliminates the cut of an integer literal and a covariable.
/// - `lit` is the integer literal.
/// - `var` is the covariable.
/// - `used_vars` are the variable names used in the top-level function we are currently in.
fn shrink_literal_var(
    lit: i64,
    var: Var,
    used_vars: &mut HashSet<Var>,
) -> axcut::syntax::Statement {
    // we bind the literal to a fresh variable ...
    let fresh_var = fresh_var(used_vars);
    axcut::syntax::statements::Literal {
        lit,
        var: fresh_var.clone(),
        next: Rc::new(
            axcut::syntax::statements::Invoke {
                var,
                // ... and wrap it into a continuation xtor
                tag: cont_int().xtors[0].name.clone(),
                ty: axcut::syntax::Ty::Decl(cont_int().name),
                args: vec![axcut::syntax::ContextBinding {
                    var: fresh_var,
                    chi: axcut::syntax::Chirality::Ext,
                    ty: axcut::syntax::Ty::I64,
                }]
                .into(),
            }
            .into(),
        ),
        free_vars_next: None,
    }
    .into()
}

/// This function translates the kinds of arithmetic binary operators from [Core](core_lang) to
/// [AxCut](axcut).
pub fn shrink_binop(op: &core_lang::syntax::BinOp) -> axcut::syntax::BinOp {
    match op {
        core_lang::syntax::BinOp::Div => axcut::syntax::BinOp::Div,
        core_lang::syntax::BinOp::Prod => axcut::syntax::BinOp::Prod,
        core_lang::syntax::BinOp::Rem => axcut::syntax::BinOp::Rem,
        core_lang::syntax::BinOp::Sum => axcut::syntax::BinOp::Sum,
        core_lang::syntax::BinOp::Sub => axcut::syntax::BinOp::Sub,
    }
}

/// This function function eliminates the cut of an arithmetic operation and a tilde-mu-binding.
/// - `fst` is the first variable of the operation.
/// - `op` is the kind of operator.
/// - `snd` is the second variable of the operation.
/// - `var` is the variable bound by the tilde-mu.
/// - `statement` is the body of the tilde-mu.
/// - `state` is the state of the whole translation.
fn shrink_op_mu(
    fst: Var,
    op: &core_lang::syntax::BinOp,
    snd: Var,
    var: Var,
    statement: Rc<FsStatement>,
    state: &mut ShrinkingState,
) -> axcut::syntax::Statement {
    axcut::syntax::statements::Op {
        fst,
        op: shrink_binop(op),
        snd,
        var,
        next: statement.shrink(state),
        free_vars_next: None,
    }
    .into()
}

/// This function function eliminates the cut of an arithmetic operation and a covariable.
/// - `fst` is the first variable of the operation.
/// - `op` is the kind of operator.
/// - `snd` is the second variable of the operation.
/// - `var` is the covariable.
/// - `used_vars` are the variable names used in the top-level function we are currently in.
fn shrink_op_var(
    fst: Var,
    op: &core_lang::syntax::BinOp,
    snd: Var,
    var: Var,
    used_vars: &mut HashSet<Var>,
) -> axcut::syntax::Statement {
    // we bind the result of the arithmetic operation to a fresh variable ...
    let fresh_var = fresh_var(used_vars);
    axcut::syntax::statements::Op {
        fst,
        op: shrink_binop(op),
        snd,
        var: fresh_var.clone(),
        next: Rc::new(
            axcut::syntax::statements::Invoke {
                var,
                // ... and wrap it into a continuation xtor
                tag: cont_int().xtors[0].name.clone(),
                ty: axcut::syntax::Ty::Decl(cont_int().name),
                args: vec![axcut::syntax::ContextBinding {
                    var: fresh_var,
                    chi: axcut::syntax::Chirality::Ext,
                    ty: axcut::syntax::Ty::I64,
                }]
                .into(),
            }
            .into(),
        ),
        free_vars_next: None,
    }
    .into()
}

impl Shrinking for FsCut {
    type Target = axcut::syntax::Statement;
    /// # Panics
    ///
    /// In this implementation of [`Shrinking::shrink`] a panic is caused if the cut it is called
    /// on is not well-typed.
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
            ) => shrink_renaming(var, variable, statement, state),

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
                FsTerm::Op(FsOp { fst, op, snd }),
                FsTerm::Mu(Mu {
                    prdcns: Cns,
                    variable,
                    statement,
                    ..
                }),
            ) => shrink_op_mu(fst, &op, snd, variable, statement, state),

            (
                FsTerm::Op(FsOp { fst, op, snd }),
                FsTerm::XVar(XVar {
                    prdcns: Cns,
                    var,
                    ty: _,
                }),
            ) => shrink_op_var(fst, &op, snd, var, state.used_vars),

            // Let
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
                args: shrink_context(args, state.codata),
                next: statement.shrink(state),
                free_vars_next: None,
            }
            .into(),

            // Invoke
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
                args: shrink_context(args, state.codata),
            }
            .into(),

            // Switch
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

            // Create
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
            ) => axcut::syntax::statements::Create {
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
            ) => axcut::syntax::statements::Create {
                var: variable,
                ty: shrink_ty(self.ty),
                context: None,
                clauses: clauses.shrink(state),
                free_vars_clauses: None,
                next: statement.shrink(state),
                free_vars_next: None,
            }
            .into(),

            // all other cases are impossible by typing
            _ => panic!("cannot happen"),
        }
    }
}
