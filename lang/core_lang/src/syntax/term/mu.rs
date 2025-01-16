use printer::{theme::ThemeExt, tokens::DOT, DocAllocator, Print};

use super::{Cns, FsTerm, Prd, PrdCns, Term, XVar};
use crate::{
    syntax::{
        statement::{FsCut, FsOp},
        Covar, FsStatement, Statement, Ty, Var,
    },
    traits::*,
};

use std::{collections::HashSet, rc::Rc};

/// Either a Mu or a TildeMu abstraction.
/// - A Mu abstraction if `T = Prd`
/// - A TildeMu abstraction if `T = Cns`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mu<T: PrdCns, S> {
    pub prdcns: T,
    pub variable: Var,
    pub statement: Rc<S>,
    pub ty: Ty,
}

impl<S> Mu<Prd, S> {
    /// Create a new Mu abstraction
    #[allow(clippy::self_named_constructors)]
    pub fn mu<T: Into<S>>(covar: &str, stmt: T, ty: Ty) -> Self {
        Mu {
            prdcns: Prd,
            variable: covar.to_string(),
            statement: Rc::new(stmt.into()),
            ty,
        }
    }
}
impl<S> Mu<Cns, S> {
    /// Create a new TildeMu abstraction
    pub fn tilde_mu<T: Into<S>>(var: &str, stmt: T, ty: Ty) -> Self {
        Mu {
            prdcns: Cns,
            variable: var.to_string(),
            statement: Rc::new(stmt.into()),
            ty,
        }
    }
}

impl<T: PrdCns> Typed for Mu<T, Statement> {
    fn get_type(&self) -> Ty {
        self.ty.clone()
    }
}

impl<T: PrdCns, S: Print> Print for Mu<T, S> {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let prefix = if self.prdcns.is_prd() {
            alloc
                .keyword("mu")
                .append(alloc.space())
                .append(self.variable.print(cfg, alloc))
                .append(DOT)
        } else {
            alloc
                .keyword("mutilde")
                .append(alloc.space())
                .append(self.variable.print(cfg, alloc))
                .append(DOT)
        };
        let tail = alloc
            .line()
            .append(self.statement.print(cfg, alloc))
            .nest(cfg.indent);
        prefix.append(tail).group()
    }
}

impl<T: PrdCns> FreeV for Mu<T, Statement> {
    fn free_vars(&self) -> HashSet<Var> {
        let mut free_vars = self.statement.free_vars();
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

impl<T: PrdCns> From<Mu<T, Statement>> for Term<T> {
    fn from(value: Mu<T, Statement>) -> Self {
        Term::Mu(value)
    }
}

impl<T: PrdCns> Subst for Mu<T, Statement> {
    type Target = Mu<T, Statement>;
    fn subst_sim(
        &self,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Mu<T, Statement> {
        let mut prod_subst_reduced: Vec<(Term<Prd>, Var)> = Vec::new();
        let mut cons_subst_reduced: Vec<(Term<Cns>, Covar)> = Vec::new();
        for subst in prod_subst {
            if subst.1 != self.variable {
                prod_subst_reduced.push(subst.clone());
            }
        }
        for subst in cons_subst {
            if subst.1 != self.variable {
                cons_subst_reduced.push(subst.clone());
            }
        }

        Mu {
            prdcns: self.prdcns.clone(),
            variable: self.variable.clone(),
            statement: self
                .statement
                .subst_sim(prod_subst_reduced.as_slice(), cons_subst_reduced.as_slice()),
            ty: self.ty.clone(),
        }
    }
}

impl<T: PrdCns> Uniquify for Mu<T, Statement> {
    fn uniquify(
        self,
        seen_vars: &mut HashSet<Var>,
        used_vars: &mut HashSet<Var>,
    ) -> Mu<T, Statement> {
        let mut new_variable = self.variable.clone();
        let mut new_statement = self.statement;
        if seen_vars.contains(&self.variable) {
            new_variable = fresh_var(used_vars, &self.variable);
            seen_vars.insert(new_variable.clone());
            if self.prdcns.is_prd() {
                new_statement = new_statement.subst_covar(
                    XVar::covar(&new_variable, self.ty.clone()).into(),
                    self.variable,
                );
            } else {
                new_statement = new_statement.subst_var(
                    XVar::var(&new_variable, self.ty.clone()).into(),
                    self.variable,
                );
            }
        } else {
            seen_vars.insert(self.variable);
        }

        Mu {
            variable: new_variable,
            statement: new_statement.uniquify(seen_vars, used_vars),
            ..self
        }
    }
}

impl<T: PrdCns> Focusing for Mu<T, Statement> {
    type Target = Mu<T, FsStatement>;
    ///N(μa.s) = μa.N(s) AND N(~μx.s) = ~μx.N(s)
    fn focus(self, state: &mut FocusingState) -> Self::Target {
        state.used_vars.insert(self.variable.clone());
        Mu {
            prdcns: self.prdcns,
            variable: self.variable,
            statement: self.statement.focus(state),
            ty: self.ty,
        }
    }
}

impl Bind for Mu<Prd, Statement> {
    ///bind(μa.s)[k] = ⟨μa.N(s) | ~μx.k(x)⟩
    ///OR (special-cased to avoid administrative redexes for arithmetic operators)
    ///bind(μa.op(p_1, p_2, a))[k] = bind(p_1)[λa1.bind(p_2)[λa_2.⊙ (a_1, a_2; ~μx.k(x))]]
    fn bind(self, k: Continuation, state: &mut FocusingState) -> FsStatement {
        state.used_vars.insert(self.variable.clone());
        let ty = self.ty.clone();
        match (*self.statement).clone() {
            Statement::Op(op)
                if *op.continuation
                    == Term::XVar(XVar {
                        prdcns: Cns,
                        ty: Ty::I64,
                        var: self.variable.clone(),
                    }) =>
            {
                let cont = Box::new(|var_fst: Var, state: &mut FocusingState| {
                    Rc::unwrap_or_clone(op.snd).bind(
                        Box::new(|var_snd: Var, state: &mut FocusingState| {
                            let new_var = state.fresh_var();
                            FsOp {
                                fst: var_fst,
                                op: op.op,
                                snd: var_snd,
                                continuation: Rc::new(
                                    Mu::tilde_mu(&new_var, k(new_var.clone(), state), Ty::I64)
                                        .into(),
                                ),
                            }
                            .into()
                        }),
                        state,
                    )
                });
                Rc::unwrap_or_clone(op.fst).bind(cont, state)
            }
            _ => {
                let new_var = state.fresh_var();
                FsCut::new(
                    self.focus(state),
                    Mu::tilde_mu(&new_var, k(new_var.clone(), state), ty.clone()),
                    ty,
                )
                .into()
            }
        }
    }
}
impl Bind for Mu<Cns, Statement> {
    ///bind(~μx.s)[k] = ⟨μa.k(a) | ~μx.N(s)⟩
    fn bind(self, k: Continuation, state: &mut FocusingState) -> FsStatement {
        state.used_vars.insert(self.variable.clone());
        let ty = self.ty.clone();
        let new_covar = state.fresh_covar();
        FsCut::new(
            Mu::mu(&new_covar, k(new_covar.clone(), state), ty.clone()),
            self.focus(state),
            ty,
        )
        .into()
    }
}

impl<T: PrdCns> From<Mu<T, FsStatement>> for FsTerm<T> {
    fn from(value: Mu<T, FsStatement>) -> Self {
        FsTerm::Mu(value)
    }
}

impl<T: PrdCns> SubstVar for Mu<T, FsStatement> {
    type Target = Mu<T, FsStatement>;
    fn subst_sim(self, subst: &[(Var, Var)]) -> Mu<T, FsStatement> {
        Mu {
            prdcns: self.prdcns,
            variable: self.variable,
            statement: self.statement.subst_sim(subst),
            ty: self.ty,
        }
    }
}

#[cfg(test)]
mod mu_tests {
    use super::{Bind, Focusing, FreeV, Subst};

    use crate::{
        syntax::{
            statement::{Cut, FsCut},
            term::{Literal, Mu, XVar},
            types::Ty,
            FsStatement, Statement,
        },
        test_common::example_subst,
    };
    use std::collections::HashSet;

    // Free variable tests

    #[test]
    fn free_vars_mu() {
        let example = Mu::mu(
            "a",
            Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64),
            Ty::I64,
        );
        let expected = HashSet::from(["x".to_string()]);
        assert_eq!(example.free_vars(), expected)
    }

    #[test]
    fn free_vars_mu_tilde() {
        let example = Mu::tilde_mu(
            "x",
            Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64),
            Ty::I64,
        );
        assert!(example.free_vars().is_empty())
    }

    #[test]
    fn free_covars_mu() {
        let example = Mu::mu(
            "a",
            Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64),
            Ty::I64,
        );
        assert!(example.free_covars().is_empty())
    }

    #[test]
    fn free_covars_mu_tilde() {
        let example = Mu::tilde_mu(
            "x",
            Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64),
            Ty::I64,
        );
        let expected = HashSet::from(["a".to_string()]);
        assert_eq!(example.free_covars(), expected)
    }

    // Substitution tests

    #[test]
    fn subst_mu() {
        let subst = example_subst();
        let result = Mu::mu(
            "a",
            Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64),
            Ty::I64,
        )
        .subst_sim(&subst.0, &subst.1);
        let expected = Mu::mu(
            "a",
            Cut::new(XVar::var("y", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64),
            Ty::I64,
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_mutilde() {
        let subst = example_subst();
        let example = Mu::tilde_mu(
            "x",
            Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64),
            Ty::I64,
        );
        let result = example.subst_sim(&subst.0, &subst.1);
        let expected = Mu::tilde_mu(
            "x",
            Cut::new(XVar::var("x", Ty::I64), XVar::covar("b", Ty::I64), Ty::I64),
            Ty::I64,
        );
        assert_eq!(result, expected)
    }

    // Focusing tests

    #[test]
    fn focus_mu1() {
        let example = Mu::mu("a", Statement::Done(Ty::I64), Ty::I64);
        let example_var = Mu::mu("a", FsStatement::Done(), Ty::I64);
        let result = example.clone().focus(&mut Default::default());
        assert_eq!(result, example_var)
    }
    #[test]
    fn focus_mu2() {
        let example = Mu::mu(
            "a",
            Cut::new(Literal::new(1), XVar::covar("a", Ty::I64), Ty::I64),
            Ty::I64,
        );
        let example_var = Mu::mu(
            "a",
            FsCut::new(Literal::new(1), XVar::covar("a", Ty::I64), Ty::I64),
            Ty::I64,
        );
        let result = example.clone().focus(&mut Default::default());
        assert_eq!(result, example_var)
    }

    #[test]
    fn bind_mu1() {
        let result = Mu::mu("a", Statement::Done(Ty::I64), Ty::I64).bind(
            Box::new(|_, _| FsStatement::Done()),
            &mut Default::default(),
        );
        let expected = FsCut::new(
            Mu::mu("a", FsStatement::Done(), Ty::I64),
            Mu::tilde_mu("x0", FsStatement::Done(), Ty::I64),
            Ty::I64,
        )
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn bind_mu2() {
        let example = Mu::mu(
            "a",
            Cut::new(Literal::new(1), XVar::covar("a", Ty::I64), Ty::I64),
            Ty::I64,
        );
        let example_var = Mu::mu(
            "a",
            FsCut::new(Literal::new(1), XVar::covar("a", Ty::I64), Ty::I64),
            Ty::I64,
        );
        let result = example.clone().bind(
            Box::new(|_, _| FsStatement::Done()),
            &mut Default::default(),
        );
        let expected = FsCut::new(
            example_var,
            Mu::tilde_mu("x0", FsStatement::Done(), Ty::I64),
            Ty::I64,
        )
        .into();
        assert_eq!(result, expected)
    }
}
