//! Defines [Op]-Terms
use printer::{
    tokens::{COMMA, DIVIDE, MINUS, MODULO, PLUS, TIMES},
    DocAllocator, Print,
};

use super::{ContextBinding, Covar, Var};
use crate::{
    syntax::{
        context::Chirality,
        fresh_var,
        statements::FsCut,
        terms::{Cns, FsStatement, FsTerm, Mu, Prd, PrdCns, Term},
        types::Ty,
    },
    traits::*,
};

use std::collections::{BTreeSet, HashSet};
use std::rc::Rc;

/// A binary operation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOp {
    /// /
    Div,
    /// *
    Prod,
    /// %
    Rem,
    /// +
    Sum,
    /// -
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

/// A binary operation on integers
/// This is always a producer
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Op {
    /// The first operand
    pub fst: Rc<Term<Prd>>,
    /// The operation
    pub op: BinOp,
    /// the second operand
    pub snd: Rc<Term<Prd>>,
}

impl Op {
    /// Creates a division from two producers
    pub fn div<T, U>(fst: T, snd: U) -> Op
    where
        T: Into<Term<Prd>>,
        U: Into<Term<Prd>>,
    {
        Op {
            fst: Rc::new(fst.into()),
            op: BinOp::Div,
            snd: Rc::new(snd.into()),
        }
    }

    /// Creates a multiplication from two producers
    pub fn prod<T, U>(fst: T, snd: U) -> Op
    where
        T: Into<Term<Prd>>,
        U: Into<Term<Prd>>,
    {
        Op {
            fst: Rc::new(fst.into()),
            op: BinOp::Prod,
            snd: Rc::new(snd.into()),
        }
    }

    ///Creates a modulo operation on two producers
    pub fn rem<T, U>(fst: T, snd: U) -> Op
    where
        T: Into<Term<Prd>>,
        U: Into<Term<Prd>>,
    {
        Op {
            fst: Rc::new(fst.into()),
            op: BinOp::Rem,
            snd: Rc::new(snd.into()),
        }
    }

    /// Crates a sum operations on two producers
    pub fn sum<T, U>(fst: T, snd: U) -> Op
    where
        T: Into<Term<Prd>>,
        U: Into<Term<Prd>>,
    {
        Op {
            fst: Rc::new(fst.into()),
            op: BinOp::Sum,
            snd: Rc::new(snd.into()),
        }
    }

    /// Crates a subtraction operation on two producers
    pub fn sub<T, U>(fst: T, snd: U) -> Op
    where
        T: Into<Term<Prd>>,
        U: Into<Term<Prd>>,
    {
        Op {
            fst: Rc::new(fst.into()),
            op: BinOp::Sub,
            snd: Rc::new(snd.into()),
        }
    }
}

impl Typed for Op {
    fn get_type(&self) -> Ty {
        Ty::I64
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
                .parens(),
        )
    }
}

impl<T: PrdCns> From<Op> for Term<T> {
    fn from(value: Op) -> Self {
        Term::Op(value)
    }
}

impl Subst for Op {
    type Target = Op;
    fn subst_sim(
        mut self,
        prod_subst: &[(Var, Term<Prd>)],
        cons_subst: &[(Covar, Term<Cns>)],
    ) -> Self::Target {
        self.fst = self.fst.subst_sim(prod_subst, cons_subst);
        self.snd = self.snd.subst_sim(prod_subst, cons_subst);

        self
    }
}

impl TypedFreeVars for Op {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        self.fst.typed_free_vars(vars);
        self.snd.typed_free_vars(vars);
    }
}

impl Uniquify for Op {
    fn uniquify(mut self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> Op {
        self.fst = self.fst.uniquify(seen_vars, used_vars);
        self.snd = self.snd.uniquify(seen_vars, used_vars);

        self
    }
}

impl Focusing for Op {
    type Target = FsTerm<Prd>;
    fn focus(self, _: &mut HashSet<Var>) -> Self::Target {
        panic!("Arithmetic operators should always be focused in cuts directly");
    }
}

impl Bind for Op {
    ///bind(⊙ (p_1, p_2))\[k\] = bind(p_1)\[λa1.bind(p_2)\[λa_2.⟨⊙ (a_1, a_2) | ~μx.k(x)⟩\]\]
    fn bind(self, k: Continuation, used_vars: &mut HashSet<Var>) -> FsStatement {
        Rc::unwrap_or_clone(self.fst).bind(
            Box::new(
                |binding_fst: ContextBinding, used_vars: &mut HashSet<Var>| {
                    Rc::unwrap_or_clone(self.snd).bind(
                        Box::new(|binding_snd, used_vars: &mut HashSet<Var>| {
                            let new_var = fresh_var(used_vars);
                            let new_binding = ContextBinding {
                                var: new_var.clone(),
                                chi: Chirality::Prd,
                                ty: Ty::I64,
                            };
                            FsCut::new(
                                FsOp {
                                    fst: binding_fst.var,
                                    op: self.op,
                                    snd: binding_snd.var,
                                },
                                Mu::tilde_mu(&new_var, k(new_binding, used_vars), Ty::I64),
                                Ty::I64,
                            )
                            .into()
                        }),
                        used_vars,
                    )
                },
            ),
            used_vars,
        )
    }
}

/// Focused binary operation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsOp {
    /// The first operand
    /// always a variable after focusing
    pub fst: Var,
    /// The operator
    pub op: BinOp,
    /// The second operand
    /// always a variable after focusing
    pub snd: Var,
}

impl Print for FsOp {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        self.fst
            .print(cfg, alloc)
            .append(alloc.space())
            .append(self.op.print(cfg, alloc))
            .append(alloc.space())
            .append(self.snd.print(cfg, alloc))
    }
}

impl<T: PrdCns> From<FsOp> for FsTerm<T> {
    fn from(value: FsOp) -> Self {
        FsTerm::Op(value)
    }
}

impl SubstVar for FsOp {
    type Target = FsOp;
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> Self::Target {
        self.fst = self.fst.subst_sim(subst);
        self.snd = self.snd.subst_sim(subst);

        self
    }
}

impl TypedFreeVars for FsOp {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        vars.insert(ContextBinding {
            var: self.fst.clone(),
            chi: Chirality::Prd,
            ty: Ty::I64,
        });
        vars.insert(ContextBinding {
            var: self.snd.clone(),
            chi: Chirality::Prd,
            ty: Ty::I64,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::{BinOp, Focusing};
    use crate::syntax::{
        statements::{Cut, FsCut},
        terms::{FsOp, Literal, Mu, Op, Prd, XVar},
        types::Ty,
        Term,
    };
    use crate::{test_common::example_subst, traits::*};
    use std::rc::Rc;

    fn example_op() -> Term<Prd> {
        Op {
            fst: Rc::new(XVar::var("x", Ty::I64).into()),
            op: BinOp::Prod,
            snd: Rc::new(XVar::var("x", Ty::I64).into()),
        }
        .into()
    }

    #[test]
    fn subst_op() {
        let subst = example_subst();
        let result = example_op().subst_sim(&subst.0, &subst.1);
        let expected = Op {
            fst: Rc::new(XVar::var("y", Ty::I64).into()),
            op: BinOp::Prod,
            snd: Rc::new(XVar::var("y", Ty::I64).into()),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_op1() {
        let result = Cut::new(
            Op {
                fst: Rc::new(Literal::new(1).into()),
                op: BinOp::Sum,
                snd: Rc::new(Literal::new(2).into()),
            },
            XVar::covar("a", Ty::I64),
            Ty::I64,
        )
        .focus(&mut Default::default());
        let expected = FsCut::new(
            Literal::new(1),
            Mu::tilde_mu(
                "x0",
                FsCut::new(
                    Literal::new(2),
                    Mu::tilde_mu(
                        "x1",
                        FsCut::new(
                            FsOp {
                                fst: "x0".to_string(),
                                op: BinOp::Sum,
                                snd: "x1".to_string(),
                            },
                            XVar::covar("a", Ty::I64),
                            Ty::I64,
                        ),
                        Ty::I64,
                    ),
                    Ty::I64,
                ),
                Ty::I64,
            ),
            Ty::I64,
        )
        .into();

        assert_eq!(result, expected)
    }

    #[test]
    fn transform_op2() {
        let result = Cut::new(
            Op {
                fst: Rc::new(XVar::var("x", Ty::I64).into()),
                op: BinOp::Prod,
                snd: Rc::new(XVar::var("y", Ty::I64).into()),
            },
            XVar::covar("a", Ty::I64),
            Ty::I64,
        )
        .focus(&mut Default::default());
        let expected = FsCut::new(
            FsOp {
                fst: "x".to_string(),
                op: BinOp::Prod,
                snd: "y".to_string(),
            },
            XVar::covar("a", Ty::I64),
            Ty::I64,
        )
        .into();
        assert_eq!(result, expected)
    }
}
