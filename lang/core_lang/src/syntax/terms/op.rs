//! This module defines arithmetic binary operations in Core.

use printer::tokens::{DIVIDE, MINUS, MODULO, PLUS, TIMES};
use printer::*;

use crate::syntax::*;
use crate::traits::*;

use std::collections::{BTreeSet, HashSet};
use std::rc::Rc;

/// This enum encodes the different kinds of arithmetic binary operators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOp {
    /// Division `/`
    Div,
    /// Multiplication `*`
    Prod,
    /// Remainder `%`
    Rem,
    /// Addition `+`
    Sum,
    /// Subtraction `-`
    Sub,
}

impl Print for BinOp {
    fn print<'a>(&'a self, _cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        match self {
            BinOp::Div => alloc.text(DIVIDE),
            BinOp::Prod => alloc.text(TIMES),
            BinOp::Rem => alloc.text(MODULO),
            BinOp::Sum => alloc.text(PLUS),
            BinOp::Sub => alloc.text(MINUS),
        }
    }
}

/// This struct defines arithmetic binary operations in Core. It consists of the input terms and the
/// kind of the binary operator. The type parameter `P` determines whether this is the unfocused
/// variant (if `P` is instantiated with [`Term<Prd>`], which is the default) or the focused
/// variant (if `P` is instantiated with [`Var`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Op<P = Rc<Term<Prd>>> {
    /// The first operand
    pub fst: P,
    /// The kind of operation
    pub op: BinOp,
    /// the second operand
    pub snd: P,
}

pub type FsOp = Op<Ident>;

impl Typed for Op {
    fn get_type(&self) -> Ty {
        Ty::I64
    }
}

impl Print for Op {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let fst = if matches!(*self.fst, Term::Op(_)) {
            self.fst.print(cfg, alloc).parens()
        } else {
            self.fst.print(cfg, alloc)
        };
        let snd = if matches!(*self.snd, Term::Op(_)) {
            self.snd.print(cfg, alloc).parens()
        } else {
            self.snd.print(cfg, alloc)
        };
        fst.group()
            .append(alloc.space())
            .append(self.op.print(cfg, alloc))
            .append(alloc.space())
            .append(snd.group())
    }
}

impl Print for FsOp {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        self.fst
            .print(cfg, alloc)
            .append(alloc.space())
            .append(self.op.print(cfg, alloc))
            .append(alloc.space())
            .append(self.snd.print(cfg, alloc))
    }
}

impl<T: Chi> From<Op> for Term<T> {
    fn from(value: Op) -> Self {
        Term::Op(value)
    }
}

impl<T: Chi> From<FsOp> for FsTerm<T> {
    fn from(value: FsOp) -> Self {
        FsTerm::Op(value)
    }
}

impl Subst for Op {
    type Target = Op;
    fn subst_sim(
        mut self,
        prod_subst: &[(Ident, Term<Prd>)],
        cons_subst: &[(Ident, Term<Cns>)],
    ) -> Self::Target {
        self.fst = self.fst.subst_sim(prod_subst, cons_subst);
        self.snd = self.snd.subst_sim(prod_subst, cons_subst);

        self
    }
}

impl SubstVar for FsOp {
    type Target = FsOp;
    fn subst_sim(mut self, subst: &[(Ident, Ident)]) -> Self::Target {
        self.fst = self.fst.subst_sim(subst);
        self.snd = self.snd.subst_sim(subst);

        self
    }
}

impl TypedFreeVars for Op {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        self.fst.typed_free_vars(vars);
        self.snd.typed_free_vars(vars);
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

impl Uniquify for Op {
    fn uniquify(mut self, state: &mut UniquifyState) -> Op {
        self.fst = self.fst.uniquify(state);
        self.snd = self.snd.uniquify(state);

        self
    }
}

impl Focusing for Op {
    type Target = FsTerm<Prd>;
    fn focus(self, _: &mut HashSet<Ident>) -> Self::Target {
        panic!("Arithmetic operators should always be focused in cuts directly");
    }
}

impl Bind for Op {
    // bind(+(p_1, p_2))[k] = bind(p_1)\[λa1.bind(p_2)[λa_2.⟨ +(a_1, a_2) | ~μx.k(x) ⟩]]
    fn bind(self, k: Continuation, used_vars: &mut HashSet<Ident>) -> FsStatement {
        Rc::unwrap_or_clone(self.fst).bind(
            Box::new(
                |binding_fst: ContextBinding, used_vars: &mut HashSet<Ident>| {
                    Rc::unwrap_or_clone(self.snd).bind(
                        Box::new(|binding_snd, used_vars: &mut HashSet<Ident>| {
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
                                Mu::tilde_mu(new_var, k(new_binding, used_vars), Ty::I64),
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

#[cfg(test)]
mod tests {
    use crate::syntax::*;
    use crate::test_common::example_subst;
    use crate::traits::*;
    extern crate self as core_lang;
    use core_macros::{covar, cut, fs_cut, fs_mutilde, fs_prod, fs_sum, id, lit, prod, sum, var};

    fn example_op() -> Term<Prd> {
        prod!(var!(id!("x")), var!(id!("x"))).into()
    }

    #[test]
    fn subst_op() {
        let subst = example_subst();
        let result = example_op().subst_sim(&subst.0, &subst.1);
        let expected = prod!(var!(id!("y")), var!(id!("y"))).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_op1() {
        let result = cut!(sum!(lit!(1), lit!(2)), covar!(id!("a"))).focus(&mut Default::default());
        let expected = fs_cut!(
            lit!(1),
            fs_mutilde!(
                id!("x"),
                fs_cut!(
                    lit!(2),
                    fs_mutilde!(
                        id!("x", 1),
                        fs_cut!(fs_sum!(id!("x"), id!("x", 1)), covar!(id!("a")))
                    )
                )
            )
        )
        .into();

        assert_eq!(result, expected)
    }

    #[test]
    fn transform_op2() {
        let result = cut!(prod!(var!(id!("x")), var!(id!("y"))), covar!(id!("a")))
            .focus(&mut Default::default());
        let expected = fs_cut!(fs_prod!(id!("x"), id!("y")), covar!(id!("a"))).into();
        assert_eq!(result, expected)
    }
}
