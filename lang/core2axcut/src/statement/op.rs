use core_lang::syntax::{
    declaration::cont_int,
    fresh_var,
    statement::{FsOp, FsStatement},
    term::{Cns, FsTerm, Mu, XVar},
};

use crate::names::translate_binop;
use crate::traits::{Shrinking, ShrinkingState};

use std::rc::Rc;

impl Shrinking for FsOp {
    type Target = axcut::syntax::Statement;

    fn shrink(self, state: &mut ShrinkingState) -> axcut::syntax::Statement {
        match Rc::unwrap_or_clone(self.continuation) {
            FsTerm::Mu(Mu {
                prdcns: Cns,
                variable,
                statement,
                ..
            }) => {
                let case = if *statement == FsStatement::Done() {
                    Rc::new(axcut::syntax::Statement::Return(
                        axcut::syntax::statements::Return {
                            var: variable.clone(),
                        },
                    ))
                } else {
                    statement.shrink(state)
                };
                axcut::syntax::Statement::Op(axcut::syntax::statements::Op {
                    fst: self.fst,
                    op: translate_binop(&self.op),
                    snd: self.snd,
                    var: variable,
                    case,
                })
            }
            FsTerm::XVar(XVar {
                prdcns: Cns,
                var,
                ty: _,
            }) => {
                let fresh_var = fresh_var(state.used_vars, "x");
                axcut::syntax::Statement::Op(axcut::syntax::statements::Op {
                    fst: self.fst,
                    op: translate_binop(&self.op),
                    snd: self.snd,
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
            _ => panic!("cannot happen"),
        }
    }
}
