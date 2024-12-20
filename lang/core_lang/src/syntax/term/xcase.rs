use printer::{
    theme::ThemeExt,
    tokens::{CASE, COCASE, COMMA, FAT_ARROW},
    util::BracesExt,
    DocAllocator, Print,
};

use super::{Cns, FsTerm, Mu, Prd, PrdCns, Term, XVar};
use crate::{
    syntax::{
        context::{Context, ContextBinding},
        statement::FsCut,
        types::Ty,
        Covar, FsStatement, Name, Statement, TypingContext, Var,
    },
    traits::*,
};

use std::{collections::HashSet, rc::Rc};

// XCase
//
//

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

impl<T: PrdCns> FreeV for XCase<T, Statement> {
    fn free_vars(&self) -> HashSet<Var> {
        self.clauses.free_vars()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        self.clauses.free_covars()
    }
}

impl<T: PrdCns> UsedBinders for XCase<T, Statement> {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.clauses.used_binders(used);
    }
}

impl<T: PrdCns> Subst for XCase<T, Statement> {
    type Target = XCase<T, Statement>;
    fn subst_sim(
        &self,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Self::Target {
        XCase {
            prdcns: self.prdcns.clone(),
            clauses: self.clauses.subst_sim(prod_subst, cons_subst),
            ty: self.ty.clone(),
        }
    }
}

impl<T: PrdCns> Uniquify for XCase<T, Statement> {
    fn uniquify(
        self,
        seen_vars: &mut HashSet<Var>,
        used_vars: &mut HashSet<Var>,
    ) -> XCase<T, Statement> {
        let seen_vars_clone = seen_vars.clone();
        let used_vars_clone = used_vars.clone();
        let clauses = self
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

        XCase { clauses, ..self }
    }
}

impl<T: PrdCns> Focusing for XCase<T, Statement> {
    type Target = XCase<T, FsStatement>;

    ///N(cocase {cases}) = cocase { N(cases) } AND N(case {cases}) = case { N(cases) }
    fn focus(self, state: &mut FocusingState) -> Self::Target {
        XCase {
            prdcns: self.prdcns,
            clauses: self.clauses.focus(state),
            ty: self.ty,
        }
    }
}

impl Bind for XCase<Prd, Statement> {
    ///bind(cocase {cases)[k] = ⟨cocase N{cases} | ~μx.k(x)⟩
    fn bind(self, k: Continuation, state: &mut FocusingState) -> FsStatement {
        let new_var = state.fresh_var();
        let cns = Mu::tilde_mu(&new_var, k(new_var.clone(), state), self.ty.clone());
        let ty = self.ty.clone();
        FsCut::new(self.focus(state), cns, ty).into()
    }
}
impl Bind for XCase<Cns, Statement> {
    ///bind(case {cases)[k] = ⟨μa.k(a) | case N{cases}⟩
    fn bind(self, k: Continuation, state: &mut FocusingState) -> FsStatement {
        let new_covar = state.fresh_covar();
        let prd = Mu::mu(&new_covar, k(new_covar.clone(), state), self.ty.clone());
        let ty = self.ty.clone();
        FsCut::new(prd, self.focus(state), ty).into()
    }
}

impl<T: PrdCns> SubstVar for XCase<T, FsStatement> {
    type Target = XCase<T, FsStatement>;
    fn subst_sim(self, subst: &[(Var, Var)]) -> Self::Target {
        XCase {
            prdcns: self.prdcns,
            clauses: self.clauses.subst_sim(subst),
            ty: self.ty,
        }
    }
}

// Clause
//
//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause<T: PrdCns, S> {
    pub prdcns: T,
    pub xtor: Name,
    pub context: TypingContext,
    pub rhs: Rc<S>,
}

impl<T: PrdCns, S: Print> Print for Clause<T, S> {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let prefix = match self.prdcns.is_prd() {
            true => alloc
                .dtor(&self.xtor)
                .append(self.context.print(cfg, alloc))
                .append(alloc.space())
                .append(FAT_ARROW),
            false => alloc
                .ctor(&self.xtor)
                .append(self.context.print(cfg, alloc))
                .append(alloc.space())
                .append(FAT_ARROW),
        };
        let tail = alloc
            .line()
            .append(self.rhs.print(cfg, alloc))
            .nest(cfg.indent);
        prefix.append(tail).group()
    }
}

pub fn print_clauses<'a, T: Print>(
    cases: &'a [T],
    cfg: &printer::PrintCfg,
    alloc: &'a printer::Alloc<'a>,
) -> printer::Builder<'a> {
    match cases.len() {
        0 => alloc.space().braces_anno(),

        1 => alloc
            .line()
            .append(cases[0].print(cfg, alloc))
            .nest(cfg.indent)
            .append(alloc.line())
            .braces_anno()
            .group(),
        _ => {
            let sep = alloc.text(COMMA).append(alloc.hardline());
            alloc
                .hardline()
                .append(alloc.intersperse(cases.iter().map(|x| x.print(cfg, alloc)), sep.clone()))
                .nest(cfg.indent)
                .append(alloc.hardline())
                .braces_anno()
        }
    }
}

impl<T: PrdCns> FreeV for Clause<T, Statement> {
    fn free_vars(self: &Clause<T, Statement>) -> HashSet<Var> {
        let mut free_vars = self.rhs.free_vars();
        for bnd in &self.context.bindings {
            if let ContextBinding::VarBinding { var, ty: _ } = bnd {
                free_vars.remove(var);
            }
        }
        free_vars
    }
    fn free_covars(self: &Clause<T, Statement>) -> HashSet<Covar> {
        let mut free_covars = self.rhs.free_covars();
        for bnd in &self.context.bindings {
            if let ContextBinding::CovarBinding { covar, ty: _ } = bnd {
                free_covars.remove(covar);
            }
        }
        free_covars
    }
}

impl<T: PrdCns> UsedBinders for Clause<T, Statement> {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        for binding in &self.context.bindings {
            match binding {
                ContextBinding::VarBinding { var, .. } => {
                    used.insert(var.clone());
                }
                ContextBinding::CovarBinding { covar, .. } => {
                    used.insert(covar.clone());
                }
            }
        }
        self.rhs.used_binders(used);
    }
}

impl<T: PrdCns> Subst for Clause<T, Statement> {
    type Target = Clause<T, Statement>;
    fn subst_sim(
        self: &Clause<T, Statement>,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Clause<T, Statement> {
        let mut prod_subst_reduced: Vec<(Term<Prd>, Var)> = Vec::new();
        let mut cons_subst_reduced: Vec<(Term<Cns>, Covar)> = Vec::new();

        for subst in prod_subst {
            if !self.context.vars().contains(&subst.1) {
                prod_subst_reduced.push(subst.clone());
            }
        }
        for subst in cons_subst {
            if !self.context.covars().contains(&subst.1) {
                cons_subst_reduced.push(subst.clone());
            }
        }

        Clause {
            prdcns: self.prdcns.clone(),
            xtor: self.xtor.clone(),
            context: self.context.clone(),
            rhs: self
                .rhs
                .subst_sim(prod_subst_reduced.as_slice(), cons_subst_reduced.as_slice()),
        }
    }
}

impl<T: PrdCns> Uniquify for Clause<T, Statement> {
    fn uniquify(
        self,
        seen_vars: &mut HashSet<Var>,
        used_vars: &mut HashSet<Var>,
    ) -> Clause<T, Statement> {
        let mut new_context: TypingContext = Context {
            bindings: Vec::new(),
        };
        let mut var_subst: Vec<(Term<Prd>, Var)> = Vec::new();
        let mut covar_subst: Vec<(Term<Cns>, Covar)> = Vec::new();

        for binding in self.context.bindings {
            match binding {
                ContextBinding::VarBinding { var, ty } => {
                    if seen_vars.contains(&var) {
                        let new_var: Var = fresh_var(used_vars, &var);
                        seen_vars.insert(new_var.clone());
                        new_context.bindings.push(ContextBinding::VarBinding {
                            var: new_var.clone(),
                            ty: ty.clone(),
                        });
                        var_subst.push((
                            XVar {
                                prdcns: Prd,
                                var: new_var,
                                ty,
                            }
                            .into(),
                            var,
                        ));
                    } else {
                        seen_vars.insert(var.clone());
                        new_context
                            .bindings
                            .push(ContextBinding::VarBinding { var, ty });
                    }
                }
                ContextBinding::CovarBinding { covar, ty } => {
                    if seen_vars.contains(&covar) {
                        let new_covar: Covar = fresh_var(used_vars, &covar);
                        seen_vars.insert(new_covar.clone());
                        new_context.bindings.push(ContextBinding::CovarBinding {
                            covar: new_covar.clone(),
                            ty: ty.clone(),
                        });
                        covar_subst.push((
                            XVar {
                                prdcns: Cns,
                                var: new_covar,
                                ty,
                            }
                            .into(),
                            covar.clone(),
                        ));
                    } else {
                        seen_vars.insert(covar.clone());
                        new_context
                            .bindings
                            .push(ContextBinding::CovarBinding { covar, ty });
                    }
                }
            }
        }

        let new_statement = if var_subst.is_empty() && covar_subst.is_empty() {
            self.rhs
        } else {
            self.rhs.subst_sim(&var_subst, &covar_subst)
        };

        Clause {
            rhs: new_statement.uniquify(seen_vars, used_vars),
            context: new_context,
            ..self
        }
    }
}

impl<T: PrdCns> Focusing for Clause<T, Statement> {
    type Target = Clause<T, FsStatement>;
    ///N(K_i(x_{i,j}) => s_i ) = K_i(x_{i,j}) => N(s_i)
    fn focus(self, state: &mut FocusingState) -> Clause<T, FsStatement> {
        state.add_context(&self.context);
        Clause {
            prdcns: self.prdcns,
            xtor: self.xtor,
            context: self.context,
            rhs: self.rhs.focus(state),
        }
    }
}

impl<T: PrdCns> SubstVar for Clause<T, FsStatement> {
    type Target = Clause<T, FsStatement>;

    fn subst_sim(self, subst: &[(Var, Var)]) -> Clause<T, FsStatement> {
        Clause {
            rhs: self.rhs.subst_sim(subst),
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::context::TypingContext;
    use crate::syntax::statement::FsCut;
    use crate::syntax::term::Prd;
    use crate::syntax::{statement::Cut, term::XVar, types::Ty};
    use crate::traits::Focusing;
    use std::rc::Rc;

    use super::Clause;

    #[test]
    fn focus_clause() {
        let mut ctx = TypingContext::empty();
        ctx.add_var("x", Ty::I64);
        ctx.add_covar("a", Ty::I64);
        let result = Clause {
            prdcns: Prd,
            xtor: "Ap".to_string(),
            context: ctx.clone(),
            rhs: Rc::new(
                Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64).into(),
            ),
        }
        .focus(&mut Default::default());
        let expected = Clause {
            prdcns: Prd,
            xtor: "Ap".to_string(),
            context: ctx,
            rhs: Rc::new(
                FsCut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64).into(),
            ),
        };
        assert_eq!(result, expected)
    }
}

#[cfg(test)]
mod testss {

    use super::{Clause, FreeV, Subst, XCase};
    use crate::{
        syntax::{
            context::TypingContext,
            statement::Cut,
            term::{Cns, Prd, XVar},
            types::Ty,
            Statement,
        },
        test_common::example_subst,
    };
    use std::{collections::HashSet, rc::Rc};

    fn example_cocase() -> XCase<Prd, Statement> {
        let mut ctx = TypingContext::empty();
        ctx.add_var("x", Ty::I64);
        ctx.add_covar("a", Ty::I64);
        XCase {
            prdcns: Prd,
            clauses: vec![
                Clause {
                    prdcns: Prd,
                    xtor: "Fst".to_string(),
                    context: ctx,
                    rhs: Rc::new(
                        Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64)
                            .into(),
                    ),
                },
                Clause {
                    prdcns: Prd,
                    xtor: "Snd".to_string(),
                    context: TypingContext::empty(),
                    rhs: Rc::new(
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
        let mut ctx = TypingContext::empty();
        ctx.add_var("x", Ty::I64);
        ctx.add_var("xs", Ty::Decl("ListInt".to_owned()));
        ctx.add_covar("a", Ty::I64);
        XCase {
            prdcns: Cns,
            clauses: vec![
                Clause {
                    prdcns: Cns,
                    xtor: "Nil".to_string(),
                    context: TypingContext::empty(),
                    rhs: Rc::new(
                        Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64)
                            .into(),
                    ),
                },
                Clause {
                    prdcns: Cns,
                    xtor: "Cons".to_string(),
                    context: ctx,
                    rhs: Rc::new(
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
    fn free_vars_cocase() {
        let result = example_cocase().free_vars();
        let expected = HashSet::from(["x".to_string()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_case() {
        let result = example_case().free_vars();
        let expected = HashSet::from(["x".to_string()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_cocase() {
        let result = example_cocase().free_covars();
        let expected = HashSet::from(["a".to_string()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_case() {
        let result = example_case().free_covars();
        let expected = HashSet::from(["a".to_string()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_case() {
        let subst = example_subst();
        let mut ctx = TypingContext::empty();
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
                    context: TypingContext::empty(),
                    rhs: Rc::new(
                        Cut::new(XVar::var("y", Ty::I64), XVar::covar("b", Ty::I64), Ty::I64)
                            .into(),
                    ),
                },
                Clause {
                    prdcns: Cns,
                    xtor: "Cons".to_string(),
                    context: ctx,
                    rhs: Rc::new(
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
        let mut ctx = TypingContext::empty();
        ctx.add_var("x", Ty::I64);
        ctx.add_covar("a", Ty::I64);
        let expected = XCase {
            prdcns: Prd,
            clauses: vec![
                Clause {
                    prdcns: Prd,
                    xtor: "Fst".to_string(),
                    context: ctx,
                    rhs: Rc::new(
                        Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64)
                            .into(),
                    ),
                },
                Clause {
                    prdcns: Prd,
                    xtor: "Snd".to_string(),
                    context: TypingContext::empty(),
                    rhs: Rc::new(
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
