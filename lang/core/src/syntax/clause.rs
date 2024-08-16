use super::{Consumer, Covar, Covariable, Producer, Statement, Var, Variable};
use crate::traits::{
    free_vars::{fresh_covar, fresh_var, FreeV},
    substitution::Subst,
};
use std::{collections::HashSet, fmt, rc::Rc};

// Clause
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause<T> {
    pub xtor: T,
    pub vars: Vec<Var>,
    pub covars: Vec<Covar>,
    pub rhs: Rc<Statement>,
}

impl<T: fmt::Display> fmt::Display for Clause<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({}; {}) => {}",
            self.xtor,
            self.vars.join(", "),
            self.covars.join(", "),
            self.rhs
        )
    }
}

impl<T> FreeV for Clause<T> {
    fn free_vars(self: &Clause<T>) -> HashSet<Var> {
        let mut free_vars = self.rhs.free_vars();
        for v in &self.vars {
            free_vars.remove(v);
        }
        free_vars
    }
    fn free_covars(self: &Clause<T>) -> HashSet<Covar> {
        let mut free_covars = self.rhs.free_covars();
        for cv in &self.covars {
            free_covars.remove(cv);
        }
        free_covars
    }
}

impl<T: Clone> Subst for Clause<T> {
    type Target = Clause<T>;
    fn subst_sim(
        self: &Clause<T>,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Clause<T> {
        let mut free_vars = self.rhs.free_vars();
        let mut free_covars = self.rhs.free_covars();
        for (prod, var) in prod_subst.iter() {
            free_vars.extend(prod.free_vars());
            free_vars.insert(var.clone());

            free_covars.extend(prod.free_covars());
        }
        for (cons, covar) in cons_subst.iter() {
            free_vars.extend(cons.free_vars());

            free_covars.extend(cons.free_covars());
            free_covars.insert(covar.clone());
        }

        let mut new_vars: Vec<Var> = vec![];
        let mut var_subst: Vec<(Producer, Var)> = vec![];

        for old_var in self.vars.iter() {
            let new_var: Var = fresh_var(&free_vars);
            free_vars.insert(new_var.clone());
            new_vars.push(new_var.clone());
            var_subst.push((Variable { var: new_var }.into(), old_var.clone()))
        }

        let mut new_covars: Vec<Covar> = vec![];
        let mut covar_subst: Vec<(Consumer, Covar)> = vec![];

        for old_covar in self.covars.iter() {
            let new_covar: Covar = fresh_covar(&free_covars);
            free_covars.insert(new_covar.clone());
            new_covars.push(new_covar.clone());
            covar_subst.push((Covariable { covar: new_covar }.into(), old_covar.clone()))
        }

        let new_statement = self.rhs.subst_sim(&var_subst, &covar_subst);

        Clause {
            xtor: self.xtor.clone(),
            vars: new_vars,
            covars: new_covars,
            rhs: new_statement.subst_sim(prod_subst, cons_subst),
        }
    }
}
