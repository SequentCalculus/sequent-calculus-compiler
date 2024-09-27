use super::{
    context::{ContextBinding, TypingContext},
    Consumer, Covar, Covariable, Name, Producer, Statement, Var, Variable,
};
use crate::traits::{
    free_vars::{fresh_covar, fresh_var, FreeV},
    substitution::Subst,
};
use std::{collections::HashSet, fmt, rc::Rc};

// Clause
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause {
    pub xtor: Name,
    pub context: TypingContext,
    pub rhs: Rc<Statement>,
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let context_strs: Vec<String> = self.context.iter().map(|bnd| bnd.to_string()).collect();
        write!(
            f,
            "{}({}) => {}",
            self.xtor,
            context_strs.join(", "),
            self.rhs
        )
    }
}

impl FreeV for Clause {
    fn free_vars(self: &Clause) -> HashSet<Var> {
        let mut free_vars = self.rhs.free_vars();
        for bnd in &self.context {
            if let ContextBinding::VarBinding { var, ty: _ } = bnd {
                free_vars.remove(var);
            }
        }
        free_vars
    }
    fn free_covars(self: &Clause) -> HashSet<Covar> {
        let mut free_covars = self.rhs.free_covars();
        for bnd in &self.context {
            if let ContextBinding::CovarBinding { covar, ty: _ } = bnd {
                free_covars.remove(covar);
            }
        }
        free_covars
    }
}

impl Subst for Clause {
    type Target = Clause;
    fn subst_sim(
        self: &Clause,
        prod_subst: &[(Producer, Var)],
        cons_subst: &[(Consumer, Covar)],
    ) -> Clause {
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

        let mut new_context: TypingContext = vec![];
        let mut var_subst: Vec<(Producer, Var)> = vec![];
        let mut covar_subst: Vec<(Consumer, Covar)> = vec![];

        for old_bnd in self.context.iter() {
            match old_bnd {
                ContextBinding::VarBinding { var, ty } => {
                    let new_var: Var = fresh_var(&free_vars);
                    free_vars.insert(new_var.clone());
                    new_context.push(ContextBinding::VarBinding {
                        var: new_var.clone(),
                        ty: ty.clone(),
                    });
                    var_subst.push((Variable { var: new_var }.into(), var.clone()));
                }
                ContextBinding::CovarBinding { covar, ty } => {
                    let new_covar: Covar = fresh_covar(&free_covars);
                    free_covars.insert(new_covar.clone());
                    new_context.push(ContextBinding::CovarBinding {
                        covar: new_covar.clone(),
                        ty: ty.clone(),
                    });
                    covar_subst.push((Covariable { covar: new_covar }.into(), covar.clone()));
                }
            }
        }

        let new_statement = self.rhs.subst_sim(&var_subst, &covar_subst);

        Clause {
            xtor: self.xtor.clone(),
            context: new_context,
            rhs: new_statement.subst_sim(prod_subst, cons_subst),
        }
    }
}
