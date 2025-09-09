//! This module defines mu- and mu-tilde-abstractions in Core.

use printer::tokens::DOT;
use printer::*;

use crate::syntax::*;
use crate::traits::*;

use std::collections::{BTreeSet, HashSet};
use std::rc::Rc;

/// This struct defines mu- and mu-tilde-abstractions in Core. It consists of the information
/// that determines whether it is in a mu (if `T` is instantiated with [`Prd`]) or a mu-tilde
/// (if `T` is instantiated with [`Cns`]), of a (co)variable bound by the abstraction, of the body,
/// and of the type. The type parameter `S` determines whether the body statement is unfocused
/// ([`Statement`]) or focused ([`FsStatement`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mu<T: PrdCns, S> {
    /// Whether we have a mu- or mu-tilde-abstraction
    pub prdcns: T,
    /// The bound (co)variable
    pub variable: Var,
    /// The body statement, either unfocused ([`Statement`]) or focused ([`FsStatement`])
    pub statement: Rc<S>,
    /// The type
    pub ty: Ty,
}

impl<S> Mu<Prd, S> {
    /// This function creates a mu-abstraction from a given covariable, body, and type.
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
    /// This function creates a mu-tilde-abstraction from a given variable, body, and type.
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
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
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
            .line_()
            .append(self.statement.print(cfg, alloc))
            .nest(cfg.indent);
        prefix.append(tail).group()
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
        mut self,
        prod_subst: &[(Var, Term<Prd>)],
        cons_subst: &[(Covar, Term<Cns>)],
    ) -> Mu<T, Statement> {
        let mut prod_subst_reduced: Vec<(Var, Term<Prd>)> = Vec::new();
        let mut cons_subst_reduced: Vec<(Covar, Term<Cns>)> = Vec::new();
        for subst in prod_subst {
            if subst.0 != self.variable {
                prod_subst_reduced.push(subst.clone());
            }
        }
        for subst in cons_subst {
            if subst.0 != self.variable {
                cons_subst_reduced.push(subst.clone());
            }
        }

        self.statement = self
            .statement
            .subst_sim(prod_subst_reduced.as_slice(), cons_subst_reduced.as_slice());
        self
    }
}

impl<T: PrdCns> TypedFreeVars for Mu<T, Statement> {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        let mut vars_statement = BTreeSet::new();
        self.statement.typed_free_vars(&mut vars_statement);

        let chi = if self.prdcns.is_prd() {
            Chirality::Cns
        } else {
            Chirality::Prd
        };
        vars_statement.remove(&ContextBinding {
            var: self.variable.clone(),
            chi,
            ty: self.ty.clone(),
        });

        vars.extend(vars_statement);
    }
}

impl<T: PrdCns> Uniquify for Mu<T, Statement> {
    fn uniquify(
        mut self,
        seen_vars: &mut HashSet<Var>,
        used_vars: &mut HashSet<Var>,
    ) -> Mu<T, Statement> {
        if seen_vars.contains(&self.variable) {
            let new_variable = fresh_name(used_vars, &self.variable);
            seen_vars.insert(new_variable.clone());
            let old_variable = self.variable;
            self.variable = new_variable;

            if self.prdcns.is_prd() {
                self.statement = self
                    .statement
                    .subst_covar(
                        old_variable,
                        XVar::covar(&self.variable, self.ty.clone()).into(),
                    )
                    .uniquify(seen_vars, used_vars);
            } else {
                self.statement = self
                    .statement
                    .subst_var(
                        old_variable,
                        XVar::var(&self.variable, self.ty.clone()).into(),
                    )
                    .uniquify(seen_vars, used_vars);
            }
        } else {
            seen_vars.insert(self.variable.clone());
            self.statement = self.statement.uniquify(seen_vars, used_vars);
        }

        self
    }
}

impl<T: PrdCns> Focusing for Mu<T, Statement> {
    type Target = Mu<T, FsStatement>;
    // focus(μa.s) = μa.focus(s) AND focus(~μx.s) = ~μx.focus(s)
    fn focus(self, used_vars: &mut HashSet<Var>) -> Self::Target {
        Mu {
            prdcns: self.prdcns,
            variable: self.variable,
            statement: self.statement.focus(used_vars),
            ty: self.ty,
        }
    }
}

impl Bind for Mu<Prd, Statement> {
    // bind(μa.s)[k] = ⟨ μa.focus(s) | ~μx.k(x) ⟩
    fn bind(self, k: Continuation, used_vars: &mut HashSet<Var>) -> FsStatement {
        let ty = self.ty.clone();
        let new_var = fresh_var(used_vars);
        let new_binding = ContextBinding {
            var: new_var.clone(),
            chi: Chirality::Prd,
            ty: ty.clone(),
        };
        FsCut::new(
            self.focus(used_vars),
            Mu::tilde_mu(&new_var, k(new_binding, used_vars), ty.clone()),
            ty,
        )
        .into()
    }
}
impl Bind for Mu<Cns, Statement> {
    // bind(~μx.s)[k] = ⟨ μa.k(a) | ~μx.focus(s) ⟩
    fn bind(self, k: Continuation, used_vars: &mut HashSet<Var>) -> FsStatement {
        let ty = self.ty.clone();
        let new_covar = fresh_covar(used_vars);
        let new_binding = ContextBinding {
            var: new_covar.clone(),
            chi: Chirality::Cns,
            ty: ty.clone(),
        };
        FsCut::new(
            Mu::mu(&new_covar, k(new_binding, used_vars), ty.clone()),
            self.focus(used_vars),
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
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> Mu<T, FsStatement> {
        self.statement = self.statement.subst_sim(subst);
        self
    }
}

impl<T: PrdCns> TypedFreeVars for Mu<T, FsStatement> {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        // all binders in focused terms are unique, so we do no need a fresh set under binders
        self.statement.typed_free_vars(vars);
        let chi = if self.prdcns.is_prd() {
            Chirality::Cns
        } else {
            Chirality::Prd
        };
        vars.remove(&ContextBinding {
            var: self.variable.clone(),
            chi,
            ty: self.ty.clone(),
        });
    }
}

#[cfg(test)]
mod mu_tests {
    use super::{Bind, Focusing, Subst};
    use crate::{
        syntax::{
            FsStatement,
            statements::{Cut, FsCut, FsExit},
            terms::{Literal, Mu, XVar},
            types::Ty,
        },
        test_common::example_subst,
    };

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
    fn focus_mu() {
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
    fn bind_mu() {
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
            Box::new(|binding, _| FsStatement::Exit(FsExit::exit(&binding.var))),
            &mut Default::default(),
        );
        let expected = FsCut::new(
            example_var,
            Mu::tilde_mu("x0", FsStatement::Exit(FsExit::exit("x0")), Ty::I64),
            Ty::I64,
        )
        .into();
        assert_eq!(result, expected)
    }
}
