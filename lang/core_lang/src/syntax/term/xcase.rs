use printer::{
    theme::ThemeExt,
    tokens::{CASE, COCASE, COMMA, FAT_ARROW},
    util::BracesExt,
    DocAllocator, Print,
};

use super::{Cns, FsTerm, Prd, PrdCns, Term, XVar};
use crate::{
    syntax::{
        context::{Context, ContextBinding},
        statement::FsCut,
        term::FsMu,
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
    pub clauses: Vec<Clause<S>>,
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
        let cns = FsMu::tilde_mu(&new_var, k(new_var.clone(), state));
        let ty = self.ty.clone();
        FsCut::new(self.focus(state), cns, ty).into()
    }
}
impl Bind for XCase<Cns, Statement> {
    ///bind(case {cases)[k] = ⟨μa.k(a) | case N{cases}⟩
    fn bind(self, k: Continuation, state: &mut FocusingState) -> FsStatement {
        let new_covar = state.fresh_covar();
        let prd = FsMu::mu(&new_covar, k(new_covar.clone(), state));
        let ty = self.ty.clone();
        FsCut::new(prd, self.focus(state), ty).into()
    }
}

impl<T: PrdCns> From<XCase<T, FsStatement>> for FsTerm<T> {
    fn from(value: XCase<T, FsStatement>) -> Self {
        FsTerm::XCase(value)
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
pub struct Clause<S> {
    pub xtor: Name,
    pub context: TypingContext,
    pub rhs: Rc<S>,
}

impl<S: Print> Print for Clause<S> {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let prefix = alloc
            .text(&self.xtor)
            .append(self.context.print(cfg, alloc))
            .append(alloc.space())
            .append(FAT_ARROW);
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

impl FreeV for Clause<Statement> {
    fn free_vars(self: &Clause<Statement>) -> HashSet<Var> {
        let mut free_vars = self.rhs.free_vars();
        for bnd in &self.context.bindings {
            if let ContextBinding::VarBinding { var, ty: _ } = bnd {
                free_vars.remove(var);
            }
        }
        free_vars
    }
    fn free_covars(self: &Clause<Statement>) -> HashSet<Covar> {
        let mut free_covars = self.rhs.free_covars();
        for bnd in &self.context.bindings {
            if let ContextBinding::CovarBinding { covar, ty: _ } = bnd {
                free_covars.remove(covar);
            }
        }
        free_covars
    }
}

impl UsedBinders for Clause<Statement> {
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

impl Subst for Clause<Statement> {
    type Target = Clause<Statement>;
    fn subst_sim(
        self: &Clause<Statement>,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Clause<Statement> {
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
            xtor: self.xtor.clone(),
            context: self.context.clone(),
            rhs: self
                .rhs
                .subst_sim(prod_subst_reduced.as_slice(), cons_subst_reduced.as_slice()),
        }
    }
}

impl Uniquify for Clause<Statement> {
    fn uniquify(
        self,
        seen_vars: &mut HashSet<Var>,
        used_vars: &mut HashSet<Var>,
    ) -> Clause<Statement> {
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

impl Focusing for Clause<Statement> {
    type Target = Clause<FsStatement>;
    ///N(K_i(x_{i,j}) => s_i ) = K_i(x_{i,j}) => N(s_i)
    fn focus(self, state: &mut FocusingState) -> Clause<FsStatement> {
        state.add_context(&self.context);
        Clause {
            xtor: self.xtor,
            context: self.context,
            rhs: self.rhs.focus(state),
        }
    }
}

impl SubstVar for Clause<FsStatement> {
    type Target = Clause<FsStatement>;

    fn subst_sim(self, subst: &[(Var, Var)]) -> Clause<FsStatement> {
        Clause {
            rhs: self.rhs.subst_sim(subst),
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::context::{Context, ContextBinding};
    use crate::syntax::statement::FsCut;
    use crate::syntax::{statement::Cut, term::XVar, types::Ty};
    use crate::traits::Focusing;
    use std::rc::Rc;

    use super::Clause;

    #[test]
    fn focus_clause() {
        let result = Clause {
            xtor: "Ap".to_string(),
            context: Context {
                bindings: vec![
                    ContextBinding::VarBinding {
                        var: "x".to_string(),
                        ty: Ty::Int,
                    },
                    ContextBinding::CovarBinding {
                        covar: "a".to_string(),
                        ty: Ty::Int,
                    },
                ],
            },
            rhs: Rc::new(
                Cut::new(XVar::var("x", Ty::Int), XVar::covar("a", Ty::Int), Ty::Int).into(),
            ),
        }
        .focus(&mut Default::default());
        let expected = Clause {
            xtor: "Ap".to_string(),
            context: Context {
                bindings: vec![
                    ContextBinding::VarBinding {
                        var: "x".to_string(),
                        ty: Ty::Int,
                    },
                    ContextBinding::CovarBinding {
                        covar: "a".to_string(),
                        ty: Ty::Int,
                    },
                ],
            },
            rhs: Rc::new(
                FsCut::new(XVar::var("x", Ty::Int), XVar::covar("a", Ty::Int), Ty::Int).into(),
            ),
        };
        assert_eq!(result, expected)
    }
}

#[cfg(test)]
mod testss {
    use printer::Print;

    use super::{Clause, Covar, FreeV, Subst, Term, Var, XCase};
    use crate::syntax::{
        context::{Context, ContextBinding},
        statement::Cut,
        term::{Cns, Prd, XVar},
        types::Ty,
        Statement,
    };
    use std::{collections::HashSet, rc::Rc};

    fn example_cocase() -> XCase<Prd, Statement> {
        XCase {
            prdcns: Prd,
            clauses: vec![
                Clause {
                    xtor: "Fst".to_string(),
                    context: Context {
                        bindings: vec![
                            ContextBinding::VarBinding {
                                var: "x".to_string(),
                                ty: Ty::Int,
                            },
                            ContextBinding::CovarBinding {
                                covar: "a".to_string(),
                                ty: Ty::Int,
                            },
                        ],
                    },
                    rhs: Rc::new(
                        Cut::new(XVar::var("x", Ty::Int), XVar::covar("a", Ty::Int), Ty::Int)
                            .into(),
                    ),
                },
                Clause {
                    xtor: "Snd".to_string(),
                    context: Context { bindings: vec![] },
                    rhs: Rc::new(
                        Cut::new(XVar::var("x", Ty::Int), XVar::covar("a", Ty::Int), Ty::Int)
                            .into(),
                    ),
                },
            ],
            ty: Ty::Decl("LPairIntInt".to_string()),
        }
        .into()
    }

    fn example_case() -> XCase<Cns, Statement> {
        XCase {
            prdcns: Cns,
            clauses: vec![
                Clause {
                    xtor: "Nil".to_string(),
                    context: Context { bindings: vec![] },
                    rhs: Rc::new(
                        Cut::new(XVar::var("x", Ty::Int), XVar::covar("a", Ty::Int), Ty::Int)
                            .into(),
                    ),
                },
                Clause {
                    xtor: "Cons".to_string(),
                    context: Context {
                        bindings: vec![
                            ContextBinding::VarBinding {
                                var: "x".to_string(),
                                ty: Ty::Int,
                            },
                            ContextBinding::VarBinding {
                                var: "xs".to_string(),
                                ty: Ty::Decl("ListInt".to_string()),
                            },
                            ContextBinding::CovarBinding {
                                covar: "a".to_string(),
                                ty: Ty::Int,
                            },
                        ],
                    },
                    rhs: Rc::new(
                        Cut::new(XVar::var("x", Ty::Int), XVar::covar("a", Ty::Int), Ty::Int)
                            .into(),
                    ),
                },
            ],
            ty: Ty::Decl("ListInt".to_string()),
        }
        .into()
    }

    fn example_prodsubst() -> Vec<(Term<Prd>, Var)> {
        vec![(XVar::var("y", Ty::Int).into(), "x".to_string())]
    }

    fn example_conssubst() -> Vec<(Term<Cns>, Covar)> {
        vec![(XVar::covar("b", Ty::Int).into(), "a".to_string())]
    }

    #[test]
    fn display_cocase() {
        let result = example_cocase().print_to_string(None);
        let expected =
            "cocase { Fst(x: Int, 'a :cns Int) => <x | 'a>, Snd => <x | 'a> }".to_string();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_case() {
        let result = example_case().print_to_string(None);
        let expected =
            "case {\n    Nil => <x | 'a>,\n    Cons(x: Int, xs: ListInt, 'a :cns Int) => <x | 'a>\n}"
                .to_string();
        assert_eq!(result, expected)
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
        let result = example_case().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = XCase {
            prdcns: Cns,
            clauses: vec![
                Clause {
                    xtor: "Nil".to_string(),
                    context: Context { bindings: vec![] },
                    rhs: Rc::new(
                        Cut::new(XVar::var("y", Ty::Int), XVar::covar("b", Ty::Int), Ty::Int)
                            .into(),
                    ),
                },
                Clause {
                    xtor: "Cons".to_string(),
                    context: Context {
                        bindings: vec![
                            ContextBinding::VarBinding {
                                var: "x".to_string(),
                                ty: Ty::Int,
                            },
                            ContextBinding::VarBinding {
                                var: "xs".to_string(),
                                ty: Ty::Decl("ListInt".to_string()),
                            },
                            ContextBinding::CovarBinding {
                                covar: "a".to_string(),
                                ty: Ty::Int,
                            },
                        ],
                    },
                    rhs: Rc::new(
                        Cut::new(XVar::var("x", Ty::Int), XVar::covar("a", Ty::Int), Ty::Int)
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
        let result = example_cocase().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = XCase {
            prdcns: Prd,
            clauses: vec![
                Clause {
                    xtor: "Fst".to_string(),
                    context: Context {
                        bindings: vec![
                            ContextBinding::VarBinding {
                                var: "x".to_string(),
                                ty: Ty::Int,
                            },
                            ContextBinding::CovarBinding {
                                covar: "a".to_string(),
                                ty: Ty::Int,
                            },
                        ],
                    },
                    rhs: Rc::new(
                        Cut::new(XVar::var("x", Ty::Int), XVar::covar("a", Ty::Int), Ty::Int)
                            .into(),
                    ),
                },
                Clause {
                    xtor: "Snd".to_string(),
                    context: Context { bindings: vec![] },
                    rhs: Rc::new(
                        Cut::new(XVar::var("y", Ty::Int), XVar::covar("b", Ty::Int), Ty::Int)
                            .into(),
                    ),
                },
            ],
            ty: Ty::Decl("LPairIntInt".to_string()),
        };
        assert_eq!(result, expected)
    }
}
