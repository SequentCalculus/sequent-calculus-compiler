use core::syntax_var::{
    cont_int,
    statement::Op,
    term::{Mu, Term, XVar},
    Chirality::Cns,
    Statement, TypeDeclaration, Var,
};
use core::traits::free_vars::fresh_var;

use crate::names::translate_binop;
use crate::traits::Shrinking;

use std::{collections::HashSet, rc::Rc};

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
            }) => {
                let case = if *statement == Statement::Done() {
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
            Term::XVar(XVar { chi: Cns, var }) => {
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

#[cfg(test)]
mod op_tests {
    use super::Shrinking;
    use std::{collections::HashSet, rc::Rc};

    #[test]
    fn shrink_mu() {
        let result = core::syntax_var::statement::Op {
            fst: "x".to_owned(),
            op: core::syntax_var::BinOp::Sum,
            snd: "y".to_owned(),
            continuation: Rc::new(
                core::syntax_var::term::Mu {
                    variable: "z".to_owned(),
                    chi: core::syntax_var::Chirality::Cns,
                    statement: Rc::new(
                        core::syntax_var::statement::Cut {
                            producer: Rc::new(core::syntax_var::term::XVar::var("z").into()),
                            ty: core::syntax_var::types::Ty::Int,
                            consumer: Rc::new(core::syntax_var::term::XVar::covar("a").into()),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .shrink(&mut HashSet::new(), &vec![]);
        let int_ty = core::syntax_var::declaration::cont_int();
        let expected = axcut::syntax::statements::Op {
            fst: "x".to_owned(),
            op: axcut::syntax::BinOp::Sum,
            snd: "y".to_owned(),
            var: "z".to_owned(),
            case: Rc::new(
                axcut::syntax::statements::Invoke {
                    var: "a".to_owned(),
                    tag: int_ty.xtors[0].name.clone(),
                    ty: axcut::syntax::Ty::Decl(int_ty.name),
                    args: vec!["z".to_owned()],
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn shrink_covar() {
        let result = core::syntax_var::statement::Op {
            fst: "x".to_owned(),
            op: core::syntax_var::BinOp::Prod,
            snd: "y".to_owned(),
            continuation: Rc::new(core::syntax_var::term::XVar::covar("a").into()),
        }
        .shrink(&mut HashSet::new(), &vec![]);
        let int_ty = core::syntax_var::declaration::cont_int();
        let expected = axcut::syntax::statements::Op {
            fst: "x".to_owned(),
            op: axcut::syntax::BinOp::Prod,
            snd: "y".to_owned(),
            var: "x0".to_owned(),
            case: Rc::new(
                axcut::syntax::statements::Invoke {
                    var: "a".to_owned(),
                    tag: int_ty.xtors[0].name.clone(),
                    ty: axcut::syntax::Ty::Decl(int_ty.name),
                    args: vec!["x0".to_owned()],
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
