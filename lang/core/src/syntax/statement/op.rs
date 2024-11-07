use printer::{
    tokens::{COMMA, SEMI},
    DocAllocator, Print,
};

use super::{Covar, Statement, Var};
use crate::{
    syntax::{
        term::{Cns, Prd, Term, XVar},
        BinOp,
    },
    traits::{
        focus::{Bind, Focusing, FocusingState},
        free_vars::FreeV,
        substitution::Subst,
    },
};
use std::{collections::HashSet, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Op {
    pub fst: Rc<Term<Prd>>,
    pub op: BinOp,
    pub snd: Rc<Term<Prd>>,
    pub continuation: Rc<Term<Cns>>,
}

impl Print for Op {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        self.op.print(cfg, alloc).append(
            self.fst
                .print(cfg, alloc)
                .append(alloc.text(COMMA))
                .append(alloc.space())
                .append(self.snd.print(cfg, alloc))
                .append(SEMI)
                .append(alloc.space())
                .append(self.continuation.print(cfg, alloc))
                .parens(),
        )
    }
}

impl FreeV for Op {
    fn free_vars(&self) -> HashSet<Var> {
        let mut free_vars = self.fst.free_vars();
        free_vars.extend(self.snd.free_vars());
        free_vars.extend(self.continuation.free_vars());
        free_vars
    }

    fn free_covars(&self) -> HashSet<Covar> {
        let mut free_covars = self.fst.free_covars();
        free_covars.extend(self.snd.free_covars());
        free_covars.extend(self.continuation.free_covars());
        free_covars
    }
}

impl From<Op> for Statement {
    fn from(value: Op) -> Self {
        Statement::Op(value)
    }
}

impl Subst for Op {
    type Target = Op;

    fn subst_sim(
        &self,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Self::Target {
        Op {
            fst: self.fst.subst_sim(prod_subst, cons_subst),
            op: self.op.clone(),
            snd: self.snd.subst_sim(prod_subst, cons_subst),
            continuation: self.continuation.subst_sim(prod_subst, cons_subst),
        }
    }
}

impl Focusing for Op {
    type Target = Statement;
    ///N(⊙(p_1, p_2; c)) = bind(p_1)[λa1.bind(p_2)[λa_2.⊙ (a_1, a_2; N(c))]]
    fn focus(self, state: &mut FocusingState) -> Statement {
        let cont = Box::new(|var1: Var, state: &mut FocusingState| {
            Rc::unwrap_or_clone(self.snd).bind(
                Box::new(|var2: Var, state: &mut FocusingState| {
                    Op {
                        fst: Rc::new(
                            XVar {
                                prdcns: Prd,
                                var: var1,
                            }
                            .into(),
                        ),
                        op: self.op,
                        snd: Rc::new(
                            XVar {
                                prdcns: Prd,
                                var: var2,
                            }
                            .into(),
                        ),
                        continuation: self.continuation.focus(state),
                    }
                    .into()
                }),
                state,
            )
        });
        Rc::unwrap_or_clone(self.fst).bind(cont, state)
    }
}

#[cfg(test)]
mod transform_tests {
    use super::Focusing;

    use crate::syntax::{
        statement::{Cut, Op},
        term::{Cns, Literal, Mu, XVar},
        BinOp,
    };
    use std::rc::Rc;

    fn example_op1() -> Op {
        Op {
            fst: Rc::new(Literal { lit: 1 }.into()),
            op: BinOp::Sum,
            snd: Rc::new(Literal { lit: 2 }.into()),
            continuation: Rc::new(XVar::covar("a").into()),
        }
    }
    fn example_op2() -> Op {
        Op {
            fst: Rc::new(XVar::var("x").into()),
            op: BinOp::Prod,
            snd: Rc::new(XVar::var("y").into()),
            continuation: Rc::new(XVar::covar("a").into()),
        }
    }

    #[test]
    fn transform_op1() {
        let result = example_op1().focus(&mut Default::default());
        let expected = Cut {
            producer: Rc::new(Literal { lit: 1 }.into()),
            consumer: Rc::new(
                Mu {
                    prdcns: Cns,
                    variable: "x0".to_owned(),
                    statement: Rc::new(
                        Cut {
                            producer: Rc::new(Literal { lit: 2 }.into()),
                            consumer: Rc::new(
                                Mu {
                                    prdcns: Cns,
                                    variable: "x1".to_owned(),
                                    statement: Rc::new(
                                        Op {
                                            fst: Rc::new(XVar::var("x0").into()),
                                            op: BinOp::Sum,
                                            snd: Rc::new(XVar::var("x1").into()),
                                            continuation: Rc::new(XVar::covar("a").into()),
                                        }
                                        .into(),
                                    ),
                                }
                                .into(),
                            ),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .into();

        assert_eq!(result, expected)
    }
    #[test]
    fn transform_op2() {
        let result = example_op2().focus(&mut Default::default());
        let expected = example_op2().into();
        assert_eq!(result, expected)
    }
}
