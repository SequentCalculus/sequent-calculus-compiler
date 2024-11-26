use core::syntax::clause::FsClause;
use core::syntax::statement::{FsCut, FsStatement};
use core::syntax::term::xcase::FsXCase;
use core::syntax::term::xtor::FsXtor;
use core::syntax::term::xvar::FsXVar;
use core::syntax::term::FsTerm;
use core::traits::substitution::SubstVar;
use core::{
    syntax::Chirality::{Cns, Prd},
    syntax::{
        term::{mu::FsMu, Literal},
        Ty,
    },
    syntax_var::{
        cont_int, context::context_vars, declaration::lookup_type_declaration, FsTypeDeclaration,
        Name, Var,
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
    statement: Rc<FsStatement>,
    ty: &Ty,
    used_vars: &mut HashSet<Var>,
    types: &[FsTypeDeclaration],
) -> axcut::syntax::Statement {
    if *ty == Ty::Int() && *statement == FsStatement::Done() {
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
    clauses: &[FsClause],
    used_vars: &mut HashSet<Var>,
    types: &[FsTypeDeclaration],
) -> axcut::syntax::Statement {
    let (statement, context) = match clauses.iter().find(
        |FsClause {
             xtor,
             context: _,
             case: _,
         }| xtor == id,
    ) {
        None => panic!("Xtor {id} not found in clauses {clauses:?}"),
        Some(FsClause {
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
    types: &[FsTypeDeclaration],
) -> axcut::syntax::Statement {
    match ty.clone() {
        Ty::Int() => axcut::syntax::Statement::Invoke(axcut::syntax::statements::Invoke {
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
    statement_prd: Rc<FsStatement>,
    var_cns: Var,
    statement_cns: Rc<FsStatement>,
    ty: &Ty,
    used_vars: &mut HashSet<Var>,
    types: &[FsTypeDeclaration],
) -> axcut::syntax::Statement {
    match ty.clone() {
        Ty::Int() => {
            let case = if *statement_cns == FsStatement::Done() {
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
    statement: Rc<FsStatement>,
    used_vars: &mut HashSet<Var>,
    types: &[FsTypeDeclaration],
) -> axcut::syntax::Statement {
    let case = if *statement == FsStatement::Done() {
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

impl Shrinking for FsCut {
    type Target = axcut::syntax::Statement;

    fn shrink(
        self,
        used_vars: &mut HashSet<Var>,
        types: &[FsTypeDeclaration],
    ) -> axcut::syntax::Statement {
        match (
            Rc::unwrap_or_clone(self.producer),
            Rc::unwrap_or_clone(self.consumer),
        ) {
            (
                FsTerm::Mu(FsMu {
                    chi: Prd,
                    variable,
                    statement,
                }),
                FsTerm::XVar(FsXVar { chi: Cns, var }),
            )
            | (
                FsTerm::XVar(FsXVar { chi: Prd, var }),
                FsTerm::Mu(FsMu {
                    chi: Cns,
                    variable,
                    statement,
                }),
            ) => shrink_renaming(var, variable, statement, &self.ty, used_vars, types),

            (FsTerm::Xtor(FsXtor { id, args }), FsTerm::XCase(FsXCase { clauses })) => {
                shrink_known_cuts(&id, args, clauses.as_slice(), used_vars, types)
            }

            (
                FsTerm::XVar(FsXVar {
                    chi: Prd,
                    var: var_prd,
                }),
                FsTerm::XVar(FsXVar {
                    chi: Cns,
                    var: var_cns,
                }),
            ) => shrink_unknown_cuts(var_prd, var_cns, self.ty, used_vars, types),

            (
                FsTerm::Mu(FsMu {
                    chi: Prd,
                    variable: var_prd,
                    statement: statement_prd,
                }),
                FsTerm::Mu(FsMu {
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
                FsTerm::Literal(Literal { lit }),
                FsTerm::Mu(FsMu {
                    chi: Cns,
                    variable,
                    statement,
                }),
            ) => shrink_literal_mu(lit, variable, statement, used_vars, types),

            (FsTerm::Literal(Literal { lit }), FsTerm::XVar(FsXVar { chi: Cns, var })) => {
                shrink_literal_var(lit, var, used_vars)
            }

            (
                FsTerm::Xtor(FsXtor { id, args }),
                FsTerm::Mu(FsMu {
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

            (FsTerm::Xtor(FsXtor { id, args }), FsTerm::XVar(FsXVar { chi: Cns, var })) => {
                axcut::syntax::Statement::Invoke(axcut::syntax::statements::Invoke {
                    var,
                    tag: id,
                    ty: translate_ty(self.ty),
                    args,
                })
            }

            (FsTerm::XVar(FsXVar { chi: Prd, var }), FsTerm::XCase(FsXCase { clauses })) => {
                axcut::syntax::Statement::Switch(axcut::syntax::statements::Switch {
                    var,
                    ty: translate_ty(self.ty),
                    clauses: clauses.shrink(used_vars, types),
                })
            }

            (
                FsTerm::Mu(FsMu {
                    chi: Prd,
                    variable,
                    statement,
                }),
                FsTerm::XCase(FsXCase { clauses }),
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
