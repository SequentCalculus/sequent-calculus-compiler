use core::syntax_var::{
    cont_int,
    statement::Op,
    term::{Mu, Term, XVar},
    Chirality::Cns,
    TypeDeclaration, Var,
};

use crate::names::translate_binop;
use crate::traits::{fresh_var, Shrinking, UsedBinders};

use std::{collections::HashSet, rc::Rc};

impl UsedBinders for Op {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.continuation.used_binders(used);
    }
}

impl Shrinking for Op {
    type Target = axcut::syntax::Statement;

    fn shrink(
        self,
        used_vars: &mut HashSet<Var>,
        types: &[TypeDeclaration],
    ) -> axcut::syntax::Statement {
        match Rc::unwrap_or_clone(self.continuation) {
            Term::Mu(Mu {
                chi: Cns,
                variable,
                statement,
            }) => axcut::syntax::Statement::Op(axcut::syntax::Op {
                fst: self.fst,
                op: translate_binop(&self.op),
                snd: self.snd,
                var: variable,
                case: statement.shrink(used_vars, types),
            }),
            Term::XVar(XVar { chi: Cns, var }) => {
                let fresh_var = fresh_var(used_vars, "x");
                axcut::syntax::Statement::Op(axcut::syntax::Op {
                    fst: self.fst,
                    op: translate_binop(&self.op),
                    snd: self.snd,
                    var: fresh_var.clone(),
                    case: Rc::new(axcut::syntax::Statement::Invoke(axcut::syntax::Invoke {
                        var,
                        tag: cont_int().xtors[0].name.clone(),
                        ty: axcut::syntax::Ty::Decl(cont_int().name),
                        args: vec![fresh_var],
                    })),
                })
            }
            _ => panic!("cannot happen"),
        }
    }
}
