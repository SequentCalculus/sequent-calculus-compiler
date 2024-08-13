use super::{Consumer, Covar, Producer, Statement, Var};
use crate::traits::{
    free_vars::{fresh_var, FreeV},
    substitution::Subst,
};
use std::{collections::HashSet, fmt, rc::Rc};

// MuTilde
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MuTilde {
    pub variable: Var,
    pub statement: Rc<Statement>,
}

impl std::fmt::Display for MuTilde {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mutilde {}. {}", self.variable, self.statement)
    }
}

impl FreeV for MuTilde {
    fn free_vars(&self) -> HashSet<Var> {
        let mut fr_st = self.statement.free_vars();
        fr_st.remove(&self.variable);
        fr_st
    }

    fn free_covars(&self) -> HashSet<Covar> {
        self.statement.free_covars()
    }
}

impl Subst for MuTilde {
    type Target = MuTilde;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        let mut fr_v: HashSet<Var> = self.statement.free_vars();
        for (prod, var) in prod_subst.iter() {
            fr_v.extend(prod.free_vars());
            fr_v.insert(var.clone());
        }
        for (cons, _) in cons_subst.iter() {
            fr_v.extend(cons.free_vars());
        }
        let new_var: Var = fresh_var(&fr_v);
        let new_st = self.statement.subst_var(
            crate::syntax::Variable {
                var: new_var.clone(),
            }
            .into(),
            self.variable.clone(),
        );
        MuTilde {
            variable: new_var,
            statement: new_st.subst_sim(prod_subst, cons_subst),
        }
    }
}

impl From<MuTilde> for Consumer {
    fn from(value: MuTilde) -> Self {
        Consumer::MuTilde(value)
    }
}
