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
};

use crate::chirality::translate_chirality;
use crate::traits::{fresh_var, Shrinking, UsedBinders};
use crate::types::translate_ty;

use std::{collections::HashSet, rc::Rc};

impl UsedBinders for Cut {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.producer.used_binders(used);
        self.consumer.used_binders(used);
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
        Ty::Int => axcut::syntax::Statement::Invoke(axcut::syntax::Invoke {
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
                        case: Rc::new(axcut::syntax::Statement::Invoke(axcut::syntax::Invoke {
                            var: var_cns.clone(),
                            tag: xtor.name.clone(),
                            ty: translate_ty(ty.clone()),
                            args: axcut::syntax::context::context_vars(&env),
                        })),
                    }
                })
                .collect();
            axcut::syntax::Statement::Switch(axcut::syntax::Switch {
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
        Ty::Int => axcut::syntax::Statement::New(axcut::syntax::New {
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
                case: statement_cns.shrink(used_vars, types),
            }],
            next: statement_prd.shrink(used_vars, types),
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
                        case: Rc::new(axcut::syntax::Statement::Leta(axcut::syntax::Leta {
                            var: var_cns.clone(),
                            ty: translate_ty(ty.clone()),
                            tag: xtor.name.clone(),
                            args: axcut::syntax::context::context_vars(&env),
                            next: statement_cns.clone().shrink(used_vars, types),
                        })),
                    }
                })
                .collect();
            axcut::syntax::Statement::New(axcut::syntax::New {
                var: var_prd,
                ty: axcut::syntax::Ty::Decl(name),
                context: None,
                clauses,
                next: statement_prd.shrink(used_vars, types),
            })
        }
    }
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
            ) => Rc::unwrap_or_clone(statement)
                .subst_sim(&[(variable, var)])
                .shrink(used_vars, types),
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
            ) => axcut::syntax::Statement::Literal(axcut::syntax::Literal {
                lit,
                var: variable,
                case: statement.shrink(used_vars, types),
            }),
            (Term::Literal(Literal { lit }), Term::XVar(XVar { chi: Cns, var })) => {
                let fresh_var = fresh_var(used_vars, "x");
                axcut::syntax::Statement::Literal(axcut::syntax::Literal {
                    lit,
                    var: fresh_var.clone(),
                    case: Rc::new(axcut::syntax::Statement::Invoke(axcut::syntax::Invoke {
                        var,
                        tag: cont_int().xtors[0].name.clone(),
                        ty: axcut::syntax::Ty::Decl(cont_int().name),
                        args: vec![fresh_var],
                    })),
                })
            }
            (
                Term::Xtor(Xtor { id, args }),
                Term::Mu(Mu {
                    chi: Cns,
                    variable,
                    statement,
                }),
            ) => axcut::syntax::Statement::Leta(axcut::syntax::Leta {
                var: variable,
                ty: translate_ty(self.ty),
                tag: id,
                args,
                next: statement.shrink(used_vars, types),
            }),
            (Term::Xtor(Xtor { id, args }), Term::XVar(XVar { chi: Cns, var })) => {
                axcut::syntax::Statement::Invoke(axcut::syntax::Invoke {
                    var,
                    tag: id,
                    ty: translate_ty(self.ty),
                    args,
                })
            }
            (Term::XVar(XVar { chi: Prd, var }), Term::XCase(XCase { clauses })) => {
                axcut::syntax::Statement::Switch(axcut::syntax::Switch {
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
            ) => axcut::syntax::Statement::New(axcut::syntax::New {
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
