use core::syntax::declaration::{cont_int, FsTypeDeclaration};
use core::syntax::statement::{FsOp, FsStatement};
use core::syntax::term::mu::FsMu;
use core::syntax::term::xvar::FsXVar;
use core::syntax::term::FsTerm;
use core::syntax::{Chirality, Var};
use core::traits::free_vars::fresh_var;

use crate::names::translate_binop;
use crate::traits::Shrinking;

use std::{collections::HashSet, rc::Rc};

impl Shrinking for FsOp {
    type Target = axcut::syntax::Statement;

    fn shrink(
        self,
        used_vars: &mut HashSet<Var>,
        types: &[FsTypeDeclaration],
    ) -> axcut::syntax::Statement {
        match Rc::unwrap_or_clone(self.continuation) {
            FsTerm::Mu(FsMu {
                chi: Chirality::Cns,
                variable,
                statement,
            }) => {
                let case = if *statement == FsStatement::Done() {
                    Rc::new(axcut::syntax::Statement::Return(
                        axcut::syntax::statements::Return {
                            var: variable.clone(),
                        },
                    ))
                } else {
                    statement.shrink(used_vars, types)
                };
                axcut::syntax::Statement::Op(axcut::syntax::statements::Op {
                    fst: self.fst,
                    op: translate_binop(&self.op),
                    snd: self.snd,
                    var: variable,
                    case,
                })
            }
            FsTerm::XVar(FsXVar {
                chi: Chirality::Cns,
                var,
            }) => {
                let fresh_var = fresh_var(used_vars, "x");
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
