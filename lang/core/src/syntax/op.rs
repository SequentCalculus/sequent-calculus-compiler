use super::{BinOp, Consumer, Covar, Producer, Statement, Var};
use crate::traits::{free_vars::FreeV, substitution::Subst};
use std::{collections::HashSet, fmt, rc::Rc};
// Op
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Op {
    pub fst: Rc<Producer>,
    pub op: BinOp,
    pub snd: Rc<Producer>,
    pub continuation: Rc<Consumer>,
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({},{};{})",
            self.op, self.fst, self.snd, self.continuation
        )
    }
}

impl FreeV for Op {
    fn free_vars(&self) -> HashSet<Var> {
        let free_p1 = self.fst.free_vars();
        let free_p2 = self.snd.free_vars();
        let free_c = self.continuation.free_vars();
        let free_p: HashSet<Var> = free_p1.union(&free_p2).cloned().collect();
        free_p.union(&free_c).cloned().collect()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        let free_p1 = self.fst.free_covars();
        let free_p2 = self.snd.free_covars();
        let free_c = self.continuation.free_covars();
        let free_p: HashSet<Covar> = free_p1.union(&free_p2).cloned().collect();
        free_p.union(&free_c).cloned().collect()
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
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        Op {
            fst: self.fst.subst_sim(prod_subst, cons_subst),
            op: self.op.clone(),
            snd: self.snd.subst_sim(prod_subst, cons_subst),
            continuation: self.continuation.subst_sim(prod_subst, cons_subst),
        }
    }
}
