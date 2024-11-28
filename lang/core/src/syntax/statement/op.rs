use printer::{
    tokens::{COMMA, DIVIDE, MINUS, MODULO, PLUS, SEMI, TIMES},
    DocAllocator, Print,
};

use super::{Covar, Statement, Var};
use crate::{
    syntax::{
        statement::FsStatement,
        term::{Cns, FsTerm, Prd, Term},
        types::Ty,
    },
    traits::*,
};

use std::{collections::HashSet, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOp {
    Div,
    Prod,
    Rem,
    Sum,
    Sub,
}

impl Print for BinOp {
    fn print<'a>(
        &'a self,
        _cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            BinOp::Div => alloc.text(DIVIDE),
            BinOp::Prod => alloc.text(TIMES),
            BinOp::Rem => alloc.text(MODULO),
            BinOp::Sum => alloc.text(PLUS),
            BinOp::Sub => alloc.text(MINUS),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Op {
    pub fst: Rc<Term<Prd>>,
    pub op: BinOp,
    pub snd: Rc<Term<Prd>>,
    pub continuation: Rc<Term<Cns>>,
}

impl Typed for Op {
    fn get_type(&self) -> Ty {
        Ty::Int
    }
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

impl From<Op> for Statement {
    fn from(value: Op) -> Self {
        Statement::Op(value)
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

impl UsedBinders for Op {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.fst.used_binders(used);
        self.snd.used_binders(used);
        self.continuation.used_binders(used);
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

impl Uniquify for Op {
    fn uniquify(self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> Op {
        Op {
            fst: self.fst.uniquify(seen_vars, used_vars),
            snd: self.snd.uniquify(seen_vars, used_vars),
            continuation: self.continuation.uniquify(seen_vars, used_vars),
            ..self
        }
    }
}

impl Focusing for Op {
    type Target = crate::syntax::statement::FsStatement;
    ///N(⊙ (p_1, p_2; c)) = bind(p_1)[λa1.bind(p_2)[λa_2.⊙ (a_1, a_2; N(c))]]
    fn focus(self, state: &mut FocusingState) -> FsStatement {
        let cont = Box::new(|var_fst: Var, state: &mut FocusingState| {
            Rc::unwrap_or_clone(self.snd).bind(
                Box::new(|var_snd: Var, state: &mut FocusingState| {
                    FsOp {
                        fst: var_fst,
                        op: self.op,
                        snd: var_snd,
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

/// Focused binary operation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsOp {
    pub fst: Var,
    pub op: BinOp,
    pub snd: Var,
    pub continuation: Rc<FsTerm>,
}

impl Print for FsOp {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        self.op.print(cfg, alloc).append(
            alloc
                .text(&self.fst)
                .append(alloc.text(COMMA))
                .append(alloc.space())
                .append(alloc.text(&self.snd))
                .append(SEMI)
                .append(alloc.space())
                .append(self.continuation.print(cfg, alloc))
                .parens(),
        )
    }
}

impl From<FsOp> for FsStatement {
    fn from(value: FsOp) -> Self {
        FsStatement::Op(value)
    }
}

impl SubstVar for FsOp {
    type Target = FsOp;

    fn subst_sim(self, subst: &[(Var, Var)]) -> Self::Target {
        FsOp {
            fst: self.fst.subst_sim(subst),
            op: self.op,
            snd: self.snd.subst_sim(subst),
            continuation: self.continuation.subst_sim(subst),
        }
    }
}

#[cfg(test)]
mod tests {
    use printer::Print;

    use super::{BinOp, Focusing};

    use crate::syntax::statement::{FsCut, FsOp};
    use crate::syntax::term::mu::FsMu;
    use crate::syntax::term::xvar::FsXVar;
    use crate::syntax::Chirality;
    use crate::syntax::{
        statement::Op,
        term::{Literal, XVar},
        types::Ty,
    };
    use std::rc::Rc;

    #[test]
    fn transform_op1() {
        let result = Op {
            fst: Rc::new(Literal::new(1).into()),
            op: BinOp::Sum,
            snd: Rc::new(Literal::new(2).into()),
            continuation: Rc::new(XVar::covar("a", Ty::Int).into()),
        }
        .focus(&mut Default::default());
        let expected = FsCut {
            producer: Rc::new(Literal { lit: 1 }.into()),
            ty: Ty::Int,
            consumer: Rc::new(
                FsMu {
                    chi: Chirality::Cns,
                    variable: "x0".to_owned(),
                    statement: Rc::new(
                        FsCut {
                            producer: Rc::new(Literal { lit: 2 }.into()),
                            ty: Ty::Int,
                            consumer: Rc::new(
                                FsMu {
                                    chi: Chirality::Cns,
                                    variable: "x1".to_owned(),
                                    statement: Rc::new(
                                        FsOp {
                                            fst: "x0".to_string(),
                                            op: BinOp::Sum,
                                            snd: "x1".to_string(),
                                            continuation: Rc::new(FsXVar::covar("a").into()),
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
        let result = Op {
            fst: Rc::new(XVar::var("x", Ty::Int).into()),
            op: BinOp::Prod,
            snd: Rc::new(XVar::var("y", Ty::Int).into()),
            continuation: Rc::new(XVar::covar("a", Ty::Int).into()),
        }
        .focus(&mut Default::default());
        let expected = FsOp {
            fst: "x".to_owned(),
            op: BinOp::Prod,
            snd: "y".to_owned(),
            continuation: Rc::new(FsXVar::covar("a").into()),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_prod() {
        assert_eq!(BinOp::Prod.print_to_string(None), "*".to_owned())
    }

    #[test]
    fn display_sum() {
        assert_eq!(BinOp::Sum.print_to_string(None), "+".to_owned())
    }

    #[test]
    fn display_sub() {
        assert_eq!(BinOp::Sub.print_to_string(None), "-".to_owned())
    }
}
