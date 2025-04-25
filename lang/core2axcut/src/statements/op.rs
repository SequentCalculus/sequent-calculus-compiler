use core_lang::syntax::{
    declaration::cont_int,
    fresh_var,
    statements::{FsOp, FsStatement},
    terms::{Cns, FsTerm, Mu, XVar},
};

use crate::shrinking::{Shrinking, ShrinkingState};

use std::rc::Rc;

pub fn shrink_binop(binop: &core_lang::syntax::BinOp) -> axcut::syntax::names::BinOp {
    match binop {
        core_lang::syntax::BinOp::Div => axcut::syntax::BinOp::Div,
        core_lang::syntax::BinOp::Prod => axcut::syntax::BinOp::Prod,
        core_lang::syntax::BinOp::Rem => axcut::syntax::BinOp::Rem,
        core_lang::syntax::BinOp::Sum => axcut::syntax::BinOp::Sum,
        core_lang::syntax::BinOp::Sub => axcut::syntax::BinOp::Sub,
    }
}

impl Shrinking for FsOp {
    type Target = axcut::syntax::Statement;

    fn shrink(self, state: &mut ShrinkingState) -> axcut::syntax::Statement {
        match Rc::unwrap_or_clone(self.next) {
            FsTerm::Mu(Mu {
                prdcns: Cns,
                variable,
                statement,
                ..
            }) => {
                let next = if *statement == FsStatement::Done() {
                    Rc::new(axcut::syntax::Statement::Exit(
                        axcut::syntax::statements::Exit {
                            var: variable.clone(),
                        },
                    ))
                } else {
                    statement.shrink(state)
                };
                axcut::syntax::Statement::Op(axcut::syntax::statements::Op {
                    fst: self.fst,
                    op: shrink_binop(&self.op),
                    snd: self.snd,
                    var: variable,
                    next,
                    free_vars_next: None,
                })
            }
            FsTerm::XVar(XVar {
                prdcns: Cns,
                var,
                ty: _,
            }) => {
                let fresh_var = fresh_var(state.used_vars);
                axcut::syntax::Statement::Op(axcut::syntax::statements::Op {
                    fst: self.fst,
                    op: shrink_binop(&self.op),
                    snd: self.snd,
                    var: fresh_var.clone(),
                    next: Rc::new(axcut::syntax::Statement::Invoke(
                        axcut::syntax::statements::Invoke {
                            var,
                            tag: cont_int().xtors[0].name.clone(),
                            ty: axcut::syntax::Ty::Decl(cont_int().name),
                            args: vec![fresh_var],
                        },
                    )),
                    free_vars_next: None,
                })
            }
            _ => panic!("cannot happen"),
        }
    }
}
