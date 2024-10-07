use super::{Consumer, Covar, Producer, Statement, Var};
use crate::{
    syntax::BinOp,
    traits::{free_vars::FreeV, substitution::Subst},
};
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
            "{}({}, {}; {})",
            self.op, self.fst, self.snd, self.continuation
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
