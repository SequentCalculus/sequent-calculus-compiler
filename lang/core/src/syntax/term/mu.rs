use printer::{
    theme::ThemeExt,
    tokens::{DOT, TICK},
    DocAllocator, Print,
};

use super::{Cns, Prd, PrdCns, Term, XVar};
use crate::{
    syntax::{
        statement::Cut,
        types::{Ty, Typed},
        Covar, Statement, Var,
    },
    traits::{
        focus::{Bind, Continuation, Focusing, FocusingState},
        free_vars::{fresh_covar, fresh_var, FreeV},
        substitution::Subst,
    },
};
use std::{collections::HashSet, rc::Rc};

/// Either a Mu or a TildeMu abstraction.
/// - A Mu abstraction if `T = Prd`
/// - A TildeMu abstraction if `T = Cns`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mu<T: PrdCns> {
    pub prdcns: T,
    pub variable: Var,
    pub statement: Rc<Statement>,
    pub ty: Ty,
}

impl Mu<Prd> {
    /// Create a new Mu abstraction
    #[allow(clippy::self_named_constructors)]
    pub fn mu<T: Into<Statement>>(covar: &str, stmt: T, ty: Ty) -> Self {
        Mu {
            prdcns: Prd,
            variable: covar.to_owned(),
            statement: Rc::new(stmt.into()),
            ty,
        }
    }
}

impl Mu<Cns> {
    /// Create a new TildeMu abstraction
    pub fn tilde_mu<T: Into<Statement>>(var: &str, stmt: T, ty: Ty) -> Self {
        Mu {
            prdcns: Cns,
            variable: var.to_owned(),
            statement: Rc::new(stmt.into()),
            ty,
        }
    }
}

impl<T: PrdCns> Typed for Mu<T> {
    fn get_type(&self) -> Ty {
        self.ty.clone()
    }
}
impl<T: PrdCns> Print for Mu<T> {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        if self.prdcns.is_prd() {
            alloc
                .keyword("mu")
                .append(alloc.space())
                .append(TICK)
                .append(self.variable.print(cfg, alloc))
                .append(DOT)
                .append(alloc.space())
                .append(self.statement.print(cfg, alloc))
        } else {
            alloc
                .keyword("mutilde")
                .append(alloc.space())
                .append(self.variable.print(cfg, alloc))
                .append(DOT)
                .append(alloc.space())
                .append(self.statement.print(cfg, alloc))
        }
    }
}

impl<T: PrdCns> FreeV for Mu<T> {
    fn free_vars(&self) -> HashSet<Var> {
        let mut free_vars = FreeV::free_vars(Rc::as_ref(&self.statement));
        if self.prdcns.is_cns() {
            free_vars.remove(&self.variable);
        }
        free_vars
    }

    fn free_covars(&self) -> HashSet<Covar> {
        let mut free_covars = self.statement.free_covars();
        if self.prdcns.is_prd() {
            free_covars.remove(&self.variable);
        }
        free_covars
    }
}

impl<T: PrdCns> From<Mu<T>> for Term<T> {
    fn from(value: Mu<T>) -> Self {
        Term::Mu(value)
    }
}

impl Subst for Mu<Prd> {
    type Target = Mu<Prd>;
    fn subst_sim(
        &self,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Mu<Prd> {
        let Mu {
            prdcns: _,
            variable,
            statement,
            ty,
        } = self;
        let mut free_covars: HashSet<Covar> = statement.free_covars();
        for (cons, covar) in cons_subst.iter() {
            free_covars.extend(cons.free_covars());
            free_covars.insert(covar.clone());
        }
        for (prod, _) in prod_subst.iter() {
            free_covars.extend(prod.free_covars());
        }
        let new_covar: Covar = fresh_covar(&free_covars);
        let new_statement: Rc<Statement> =
            statement.subst_covar(XVar::covar(&new_covar, ty.clone()).into(), variable.clone());
        Mu {
            prdcns: Prd,
            variable: new_covar,
            statement: new_statement.subst_sim(prod_subst, cons_subst),
            ty: ty.clone(),
        }
    }
}

impl Subst for Mu<Cns> {
    type Target = Mu<Cns>;
    fn subst_sim(
        &self,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Mu<Cns> {
        let Mu {
            prdcns: _,
            variable,
            statement,
            ty,
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
        let new_statement: Rc<Statement> =
            statement.subst_var(XVar::var(&new_var, ty.clone()).into(), variable.clone());
        Mu {
            prdcns: Cns,
            variable: new_var,
            statement: new_statement.subst_sim(prod_subst, cons_subst),
            ty: ty.clone(),
        }
    }
}

impl<T: PrdCns> Focusing for Mu<T> {
    type Target = Mu<T>;
    ///N(μa.s) = μa.N(s)
    fn focus(self, state: &mut FocusingState) -> Self::Target {
        state.used_covars.insert(self.variable.clone());
        Mu {
            prdcns: self.prdcns,
            variable: self.variable,
            statement: self.statement.focus(state),
            ty: self.ty,
        }
    }
}

impl Bind for Mu<Prd> {
    ///bind(μa.s)[k] = ⟨μa.N(s) | ~μx.k(x)⟩
    fn bind(self, k: Continuation, state: &mut FocusingState) -> Statement {
        state.used_covars.insert(self.variable.clone());
        let new_var = state.fresh_var();
        let ty = self.ty.clone();
        Cut::new(
            self.focus(state),
            Mu::tilde_mu(&new_var, k(new_var.clone(), state), ty.clone()),
            ty,
        )
        .into()
    }
}

impl Bind for Mu<Cns> {
    /// bind(~μx.s)[k] = ⟨μa.k(a) | ~μx.N(s)⟩
    fn bind(self, k: Continuation, state: &mut FocusingState) -> Statement {
        state.used_vars.insert(self.variable.clone());
        let new_covar = state.fresh_covar();
        let ty = self.ty.clone();
        Cut::new(
            Mu::mu(&new_covar, k(new_covar.clone(), state), ty.clone()),
            self.focus(state),
            ty,
        )
        .into()
    }
}

#[cfg(test)]
mod mu_tests {
    use printer::Print;

    use super::{Bind, Focusing};

    use super::{FreeV, Mu, Subst, Term};
    use crate::syntax::{
        statement::Cut,
        term::{Cns, Literal, Prd, XVar},
        types::Ty,
        Statement,
    };
    use crate::syntax::{Covar, Var};
    use std::collections::HashSet;

    // Display Tests

    #[test]
    fn display_mu() {
        let example = Mu::mu(
            "a",
            Cut::new(
                XVar::var("x", Ty::Int()),
                XVar::covar("a", Ty::Int()),
                Ty::Int(),
            ),
            Ty::Int(),
        );
        let result = example.print_to_string(None);
        let expected = "mu 'a. <x | 'a>".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_mu_tilde() {
        let example = Mu::tilde_mu(
            "x",
            Cut::new(
                XVar::var("x", Ty::Int()),
                XVar::covar("a", Ty::Int()),
                Ty::Int(),
            ),
            Ty::Int(),
        );
        let result = example.print_to_string(None);
        let expected = "mutilde x. <x | 'a>".to_owned();
        assert_eq!(result, expected)
    }

    // Free variable tests

    #[test]
    fn free_vars_mu() {
        let example = Mu::mu(
            "a",
            Cut::new(
                XVar::var("x", Ty::Int()),
                XVar::covar("a", Ty::Int()),
                Ty::Int(),
            ),
            Ty::Int(),
        );
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(example.free_vars(), expected)
    }

    #[test]
    fn free_vars_mu_tilde() {
        let example = Mu::tilde_mu(
            "x",
            Cut::new(
                XVar::var("x", Ty::Int()),
                XVar::covar("a", Ty::Int()),
                Ty::Int(),
            ),
            Ty::Int(),
        );
        assert!(example.free_vars().is_empty())
    }

    #[test]
    fn free_covars_mu() {
        let example = Mu::mu(
            "a",
            Cut::new(
                XVar::var("x", Ty::Int()),
                XVar::covar("a", Ty::Int()),
                Ty::Int(),
            ),
            Ty::Int(),
        );
        assert!(example.free_covars().is_empty())
    }

    #[test]
    fn free_covars_mu_tilde() {
        let example = Mu::tilde_mu(
            "x",
            Cut::new(
                XVar::var("x", Ty::Int()),
                XVar::covar("a", Ty::Int()),
                Ty::Int(),
            ),
            Ty::Int(),
        );
        let expected = HashSet::from(["a".to_owned()]);
        assert_eq!(example.free_covars(), expected)
    }

    // Substitution tests

    #[test]
    fn subst_mu() {
        let prd_subst: Vec<(Term<Prd>, Var)> =
            vec![(XVar::var("y", Ty::Int()).into(), "x".to_owned())];
        let cns_subst: Vec<(Term<Cns>, Covar)> =
            vec![(XVar::covar("b", Ty::Int()).into(), "a".to_owned())];
        let result = Mu::mu(
            "a",
            Cut::new(
                XVar::var("x", Ty::Int()),
                XVar::covar("a", Ty::Int()),
                Ty::Int(),
            ),
            Ty::Int(),
        )
        .subst_sim(&prd_subst, &cns_subst);
        let expected = Mu::mu(
            "a0",
            Cut::new(
                XVar::var("y", Ty::Int()),
                XVar::covar("a0", Ty::Int()),
                Ty::Int(),
            ),
            Ty::Int(),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_mutilde() {
        let prd_subst: Vec<(Term<Prd>, Var)> =
            vec![(XVar::var("y", Ty::Int()).into(), "x".to_owned())];
        let cns_subst: Vec<(Term<Cns>, Covar)> =
            vec![(XVar::covar("b", Ty::Int()).into(), "a".to_owned())];
        let example = Mu::tilde_mu(
            "x",
            Cut::new(
                XVar::var("x", Ty::Int()),
                XVar::covar("a", Ty::Int()),
                Ty::Int(),
            ),
            Ty::Int(),
        );
        let result = example.subst_sim(&prd_subst, &cns_subst);
        let expected = Mu::tilde_mu(
            "x0",
            Cut::new(
                XVar::var("x0", Ty::Int()),
                XVar::covar("b", Ty::Int()),
                Ty::Int(),
            ),
            Ty::Int(),
        );
        assert_eq!(result, expected)
    }

    // Focusing tests

    #[test]
    fn focus_mu1() {
        let ex = Mu::mu("a", Statement::Done(Ty::Int()), Ty::Int());
        let result = ex.clone().focus(&mut Default::default());
        assert_eq!(result, ex)
    }
    #[test]
    fn focus_mu2() {
        let example = Mu::mu(
            "a",
            Cut::new(Literal::new(1), XVar::covar("a", Ty::Int()), Ty::Int()),
            Ty::Int(),
        );
        let result = example.clone().focus(&mut Default::default());
        assert_eq!(result, example)
    }

    #[test]
    fn bind_mu1() {
        let result = Mu::mu("a", Statement::Done(Ty::Int()), Ty::Int()).bind(
            Box::new(|_, _| Statement::Done(Ty::Int())),
            &mut Default::default(),
        );
        let expected = Cut::new(
            Mu::mu("a", Statement::Done(Ty::Int()), Ty::Int()),
            Mu::tilde_mu("x0", Statement::Done(Ty::Int()), Ty::Int()),
            Ty::Int(),
        )
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn bind_mu2() {
        let example = Mu::mu(
            "a",
            Cut::new(Literal::new(1), XVar::covar("a", Ty::Int()), Ty::Int()),
            Ty::Int(),
        );
        let result = example.clone().bind(
            Box::new(|_, _| Statement::Done(Ty::Int())),
            &mut Default::default(),
        );
        let expected = Cut::new(
            example,
            Mu::tilde_mu("x0", Statement::Done(Ty::Int()), Ty::Int()),
            Ty::Int(),
        )
        .into();
        assert_eq!(result, expected)
    }
}
