use printer::{
    theme::ThemeExt,
    tokens::{CASE, COCASE},
    util::BracesExt,
    DocAllocator, Print,
};

use super::{print_clauses, Clause, Cns, ContextBinding, FsTerm, Mu, Prd, PrdCns, Term};
use crate::{
    syntax::{
        fresh_covar, fresh_var, statements::FsCut, types::Ty, Covar, FsStatement, Statement, Var,
    },
    traits::*,
};

use std::collections::{BTreeSet, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XCase<T: PrdCns, S> {
    pub prdcns: T,
    pub clauses: Vec<Clause<T, S>>,
    pub ty: Ty,
}

impl<T: PrdCns, S> Typed for XCase<T, S> {
    fn get_type(&self) -> Ty {
        self.ty.clone()
    }
}

impl<T: PrdCns, S: Print> Print for XCase<T, S> {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        if self.prdcns.is_prd() {
            alloc.keyword(COCASE).append(alloc.space()).append(
                alloc
                    .space()
                    .append(self.clauses.print(cfg, alloc))
                    .append(alloc.space())
                    .braces_anno(),
            )
        } else {
            alloc
                .keyword(CASE)
                .append(alloc.space())
                .append(print_clauses(&self.clauses, cfg, alloc))
        }
    }
}

impl<T: PrdCns> From<XCase<T, Statement>> for Term<T> {
    fn from(value: XCase<T, Statement>) -> Self {
        Term::XCase(value)
    }
}

impl<T: PrdCns> From<XCase<T, FsStatement>> for FsTerm<T> {
    fn from(value: XCase<T, FsStatement>) -> Self {
        FsTerm::XCase(value)
    }
}

impl<T: PrdCns> Subst for XCase<T, Statement> {
    type Target = XCase<T, Statement>;
    fn subst_sim(
        mut self,
        prod_subst: &[(Var, Term<Prd>)],
        cons_subst: &[(Covar, Term<Cns>)],
    ) -> Self::Target {
        self.clauses = self.clauses.subst_sim(prod_subst, cons_subst);
        self
    }
}

impl<T: PrdCns> TypedFreeVars for XCase<T, Statement> {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>, state: &TypedFreeVarsState) {
        self.clauses.typed_free_vars(vars, state);
    }
}

impl<T: PrdCns> Uniquify for XCase<T, Statement> {
    fn uniquify(
        mut self,
        seen_vars: &mut HashSet<Var>,
        used_vars: &mut HashSet<Var>,
    ) -> XCase<T, Statement> {
        let seen_vars_clone = seen_vars.clone();
        let used_vars_clone = used_vars.clone();
        self.clauses = self
            .clauses
            .into_iter()
            .map(|clause| {
                let mut seen_vars_clause = seen_vars_clone.clone();
                let mut used_vars_clause = used_vars_clone.clone();
                let clause = clause.uniquify(&mut seen_vars_clause, &mut used_vars_clause);
                seen_vars.extend(seen_vars_clause);
                used_vars.extend(used_vars_clause);
                clause
            })
            .collect();

        self
    }
}

impl<T: PrdCns> Focusing for XCase<T, Statement> {
    type Target = XCase<T, FsStatement>;
    ///N(cocase {cases}) = cocase { N(cases) } AND N(case {cases}) = case { N(cases) }
    fn focus(self, used_vars: &mut HashSet<Var>) -> Self::Target {
        XCase {
            prdcns: self.prdcns,
            clauses: self.clauses.focus(used_vars),
            ty: self.ty,
        }
    }
}

impl Bind for XCase<Prd, Statement> {
    ///bind(cocase {cases)[k] = ⟨cocase N{cases} | ~μx.k(x)⟩
    fn bind(self, k: Continuation, used_vars: &mut HashSet<Var>) -> FsStatement {
        let new_var = fresh_var(used_vars);
        let cns = Mu::tilde_mu(&new_var, k(new_var.clone(), used_vars), self.ty.clone());
        let ty = self.ty.clone();
        FsCut::new(self.focus(used_vars), cns, ty).into()
    }
}
impl Bind for XCase<Cns, Statement> {
    ///bind(case {cases)[k] = ⟨μa.k(a) | case N{cases}⟩
    fn bind(self, k: Continuation, used_vars: &mut HashSet<Var>) -> FsStatement {
        let new_covar = fresh_covar(used_vars);
        let prd = Mu::mu(&new_covar, k(new_covar.clone(), used_vars), self.ty.clone());
        let ty = self.ty.clone();
        FsCut::new(prd, self.focus(used_vars), ty).into()
    }
}

impl<T: PrdCns> SubstVar for XCase<T, FsStatement> {
    type Target = XCase<T, FsStatement>;
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> Self::Target {
        self.clauses = self.clauses.subst_sim(subst);
        self
    }
}

impl<T: PrdCns> TypedFreeVars for XCase<T, FsStatement> {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>, state: &TypedFreeVarsState) {
        self.clauses.typed_free_vars(vars, state);
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::context::TypingContext;
    use crate::traits::Focusing;

    use super::{Clause, Subst, XCase};
    use crate::{
        syntax::{
            statements::{Cut, FsCut},
            terms::{Cns, Prd, XVar},
            types::Ty,
            Statement,
        },
        test_common::example_subst,
    };

    use std::rc::Rc;

    #[test]
    fn focus_clause() {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::I64);
        ctx.add_covar("a", Ty::I64);
        let result = Clause {
            prdcns: Prd,
            xtor: "Apply".to_string(),
            context: ctx.clone(),
            body: Rc::new(
                Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64).into(),
            ),
        }
        .focus(&mut Default::default());
        let expected = Clause {
            prdcns: Prd,
            xtor: "Apply".to_string(),
            context: ctx,
            body: Rc::new(
                FsCut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64).into(),
            ),
        };
        assert_eq!(result, expected)
    }

    fn example_cocase() -> XCase<Prd, Statement> {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::I64);
        ctx.add_covar("a", Ty::I64);
        XCase {
            prdcns: Prd,
            clauses: vec![
                Clause {
                    prdcns: Prd,
                    xtor: "Fst".to_string(),
                    context: ctx,
                    body: Rc::new(
                        Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64)
                            .into(),
                    ),
                },
                Clause {
                    prdcns: Prd,
                    xtor: "Snd".to_string(),
                    context: TypingContext::default(),
                    body: Rc::new(
                        Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64)
                            .into(),
                    ),
                },
            ],
            ty: Ty::Decl("LPairIntInt".to_string()),
        }
        .into()
    }

    fn example_case() -> XCase<Cns, Statement> {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::I64);
        ctx.add_var("xs", Ty::Decl("ListInt".to_owned()));
        ctx.add_covar("a", Ty::I64);
        XCase {
            prdcns: Cns,
            clauses: vec![
                Clause {
                    prdcns: Cns,
                    xtor: "Nil".to_string(),
                    context: TypingContext::default(),
                    body: Rc::new(
                        Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64)
                            .into(),
                    ),
                },
                Clause {
                    prdcns: Cns,
                    xtor: "Cons".to_string(),
                    context: ctx,
                    body: Rc::new(
                        Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64)
                            .into(),
                    ),
                },
            ],
            ty: Ty::Decl("ListInt".to_string()),
        }
        .into()
    }

    #[test]
    fn subst_case() {
        let subst = example_subst();
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::I64);
        ctx.add_var("xs", Ty::Decl("ListInt".to_owned()));
        ctx.add_covar("a", Ty::I64);
        let result = example_case().subst_sim(&subst.0, &subst.1);
        let expected = XCase {
            prdcns: Cns,
            clauses: vec![
                Clause {
                    prdcns: Cns,
                    xtor: "Nil".to_string(),
                    context: TypingContext::default(),
                    body: Rc::new(
                        Cut::new(XVar::var("y", Ty::I64), XVar::covar("b", Ty::I64), Ty::I64)
                            .into(),
                    ),
                },
                Clause {
                    prdcns: Cns,
                    xtor: "Cons".to_string(),
                    context: ctx,
                    body: Rc::new(
                        Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64)
                            .into(),
                    ),
                },
            ],
            ty: Ty::Decl("ListInt".to_string()),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_cocase() {
        let subst = example_subst();
        let result = example_cocase().subst_sim(&subst.0, &subst.1);
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::I64);
        ctx.add_covar("a", Ty::I64);
        let expected = XCase {
            prdcns: Prd,
            clauses: vec![
                Clause {
                    prdcns: Prd,
                    xtor: "Fst".to_string(),
                    context: ctx,
                    body: Rc::new(
                        Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64)
                            .into(),
                    ),
                },
                Clause {
                    prdcns: Prd,
                    xtor: "Snd".to_string(),
                    context: TypingContext::default(),
                    body: Rc::new(
                        Cut::new(XVar::var("y", Ty::I64), XVar::covar("b", Ty::I64), Ty::I64)
                            .into(),
                    ),
                },
            ],
            ty: Ty::Decl("LPairIntInt".to_string()),
        };
        assert_eq!(result, expected)
    }
}
