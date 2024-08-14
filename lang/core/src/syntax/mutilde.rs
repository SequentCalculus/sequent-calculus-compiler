use super::{Consumer, Covar, Producer, Statement, Var, Variable};
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
        let mut free_vars = self.statement.free_vars();
        free_vars.remove(&self.variable);
        free_vars
    }

    fn free_covars(&self) -> HashSet<Covar> {
        self.statement.free_covars()
    }
}

impl From<MuTilde> for Consumer {
    fn from(value: MuTilde) -> Self {
        Consumer::MuTilde(value)
    }
}

impl Subst for MuTilde {
    type Target = MuTilde;

    fn subst_sim(
        &self,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Self::Target {
        let MuTilde {
            variable,
            statement,
        } = self;
        let mut free_vars: HashSet<Var> = statement.free_vars();
        for (prod, var) in prod_subst.iter() {
            free_vars.extend(prod.free_vars());
            free_vars.insert(var.clone());
        }
        for (cons, _) in cons_subst.iter() {
            free_vars.extend(cons.free_vars());
        }
        let new_var: Var = fresh_var(&free_vars);
        let new_statement: Rc<Statement> = statement.subst_var(
            Variable {
                var: new_var.clone(),
            }
            .into(),
            variable.clone(),
        );
        MuTilde {
            variable: new_var,
            statement: new_statement.subst_sim(prod_subst, cons_subst),
        }
    }
}
