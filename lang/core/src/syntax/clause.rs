use super::{Consumer, Covar, Covariable, Producer, Statement, Var};
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
            "{}({};{}) => {}",
            self.xtor,
            self.vars.join(", "),
            self.covars.join(", "),
            self.rhs
        )
    }
}

impl<T> FreeV for Clause<T> {
    fn free_vars(self: &Clause<T>) -> HashSet<Var> {
        let free_pt = self.rhs.free_vars();
        let unfree = HashSet::from_iter(self.vars.iter().cloned());
        free_pt.difference(&unfree).cloned().collect()
    }
    fn free_covars(self: &Clause<T>) -> HashSet<Covar> {
        let free_pt = self.rhs.free_covars();
        let unfree = HashSet::from_iter(self.covars.iter().cloned());
        free_pt.difference(&unfree).cloned().collect()
    }
}

impl<T: Clone> Subst for Clause<T> {
    type Target = Clause<T>;
    fn subst_sim(
        self: &Clause<T>,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Clause<T> {
        let mut fr_v = self.rhs.free_vars();
        let mut fr_cv = self.rhs.free_covars();
        for (prod, var) in prod_subst.iter() {
            fr_v.extend(prod.free_vars());
            fr_v.insert(var.clone());

            fr_cv.extend(prod.free_covars());
        }
        for (cons, covar) in cons_subst.iter() {
            fr_v.extend(cons.free_vars());

            fr_cv.extend(cons.free_covars());
            fr_cv.insert(covar.clone());
        }

        let mut new_vars: Vec<Var> = vec![];
        let mut var_subst: Vec<(Producer, Var)> = vec![];

        for old_var in self.vars.iter() {
            let new_var: Var = fresh_var(&fr_v);
            fr_v.insert(new_var.clone());
            new_vars.insert(0, new_var.clone());
            var_subst.insert(
                0,
                (
                    crate::syntax::Variable { var: new_var }.into(),
                    old_var.clone(),
                ),
            )
        }

        let mut new_covars: Vec<Covar> = vec![];
        let mut covar_subst: Vec<(Consumer, Covar)> = vec![];

        for old_covar in self.covars.iter() {
            let new_covar: Covar = fresh_covar(&fr_cv);
            fr_cv.insert(new_covar.clone());
            new_covars.insert(0, new_covar.clone());
            covar_subst.insert(
                0,
                (Covariable { covar: new_covar }.into(), old_covar.clone()),
            )
        }

        let new_st = self.rhs.subst_sim(&var_subst, &covar_subst);

        Clause {
            xtor: self.xtor.clone(),
            vars: new_vars,
            covars: new_covars,
            rhs: new_st.subst_sim(prod_subst, cons_subst),
        }
    }
}
