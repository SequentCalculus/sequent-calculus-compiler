use printer::{tokens::FAT_ARROW, DocAllocator, Print};

use super::{
    context::{ContextBinding, TypingContext},
    term::{Cns, Prd, Term, XVar},
    Covar, Name, Statement, Var,
};
use crate::traits::{
    focus::{Focusing, FocusingState},
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

impl Print for Clause {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .text(&self.xtor)
            .append(self.context.print(cfg, alloc).parens())
            .append(FAT_ARROW)
            .append(self.rhs.print(cfg, alloc))
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
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
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
        let mut var_subst: Vec<(Term<Prd>, Var)> = vec![];
        let mut covar_subst: Vec<(Term<Cns>, Covar)> = vec![];

        for old_bnd in self.context.iter() {
            match old_bnd {
                ContextBinding::VarBinding { var, ty } => {
                    let new_var: Var = fresh_var(&free_vars);
                    free_vars.insert(new_var.clone());
                    new_context.push(ContextBinding::VarBinding {
                        var: new_var.clone(),
                        ty: ty.clone(),
                    });
                    var_subst.push((
                        XVar {
                            prdcns: Prd,
                            var: new_var,
                        }
                        .into(),
                        var.clone(),
                    ));
                }
                ContextBinding::CovarBinding { covar, ty } => {
                    let new_covar: Covar = fresh_covar(&free_covars);
                    free_covars.insert(new_covar.clone());
                    new_context.push(ContextBinding::CovarBinding {
                        covar: new_covar.clone(),
                        ty: ty.clone(),
                    });
                    covar_subst.push((
                        XVar {
                            prdcns: Cns,
                            var: new_covar,
                        }
                        .into(),
                        covar.clone(),
                    ));
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

impl Focusing for Clause {
    type Target = Clause;
    ///N(K_i(x_{i,j}; a_{i,j}) => s_i ) = K_i (x_{i,j}; a_{i,j} ) => N(s_i)
    fn focus(self, state: &mut FocusingState) -> Clause {
        state.add_context(&self.context);
        Clause {
            xtor: self.xtor,
            context: self.context,
            rhs: self.rhs.focus(state),
        }
    }
}

#[cfg(test)]
mod transform_tests {
    use super::Focusing;
    use crate::syntax::{
        context::ContextBinding,
        statement::Cut,
        term::{Cns, Prd, XVar},
        types::Ty,
        Clause,
    };
    use std::rc::Rc;

    fn example_clause1() -> Clause {
        Clause {
            xtor: "Tup".to_owned(),
            context: vec![
                ContextBinding::VarBinding {
                    var: "x".to_owned(),
                    ty: Ty::Int(),
                },
                ContextBinding::VarBinding {
                    var: "y".to_owned(),
                    ty: Ty::Int(),
                },
                ContextBinding::CovarBinding {
                    covar: "a".to_owned(),
                    ty: Ty::Int(),
                },
            ],
            rhs: Rc::new(
                Cut {
                    producer: Rc::new(
                        XVar {
                            prdcns: Prd,
                            var: "x".to_owned(),
                        }
                        .into(),
                    ),
                    consumer: Rc::new(
                        XVar {
                            prdcns: Cns,
                            var: "a".to_owned(),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
    }
    fn example_clause2() -> Clause {
        Clause {
            xtor: "Ap".to_owned(),
            context: vec![
                ContextBinding::VarBinding {
                    var: "x".to_owned(),
                    ty: Ty::Int(),
                },
                ContextBinding::CovarBinding {
                    covar: "a".to_owned(),
                    ty: Ty::Int(),
                },
            ],
            rhs: Rc::new(
                Cut {
                    producer: Rc::new(
                        XVar {
                            prdcns: Prd,
                            var: "x".to_owned(),
                        }
                        .into(),
                    ),
                    consumer: Rc::new(
                        XVar {
                            prdcns: Cns,
                            var: "a".to_owned(),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
    }

    #[test]
    fn transform_clause1() {
        let result = example_clause1().focus(&mut Default::default());
        let expected = example_clause1();
        assert_eq!(result, expected)
    }
    #[test]
    fn transform_clause2() {
        let result = example_clause2().focus(&mut Default::default());
        let expected = example_clause2();
        assert_eq!(result, expected)
    }
}
