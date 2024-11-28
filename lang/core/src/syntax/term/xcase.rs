use printer::{
    theme::ThemeExt,
    tokens::{CASE, COCASE, COMMA, FAT_ARROW},
    util::BracesExt,
    DocAllocator, Print,
};

use super::{Cns, FsTerm, Prd, PrdCns, Term, XVar};
use crate::{
    syntax::{
        context::{Context, ContextBinding, FsTypingContext},
        statement::FsStatement,
        types::Ty,
        Covar, Name, Statement, TypingContext, Var,
    },
    traits::*,
};

use std::{collections::HashSet, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XCase<T: PrdCns> {
    pub prdcns: T,
    pub clauses: Vec<Clause>,
    pub ty: Ty,
}

impl<T: PrdCns> Typed for XCase<T> {
    fn get_type(&self) -> Ty {
        self.ty.clone()
    }
}

impl<T: PrdCns> Print for XCase<T> {
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

impl<T: PrdCns> From<XCase<T>> for Term<T> {
    fn from(value: XCase<T>) -> Self {
        Term::XCase(value)
    }
}

impl<T: PrdCns> FreeV for XCase<T> {
    fn free_vars(&self) -> HashSet<Var> {
        self.clauses.free_vars()
    }

    fn free_covars(&self) -> HashSet<Covar> {
        self.clauses.free_covars()
    }
}

impl<T: PrdCns> UsedBinders for XCase<T> {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.clauses.used_binders(used);
    }
}

impl<T: PrdCns> Subst for XCase<T> {
    type Target = XCase<T>;
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

impl<T: PrdCns> Uniquify for XCase<T> {
    fn uniquify(self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> XCase<T> {
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

impl<T: PrdCns> Focusing for XCase<T> {
    type Target = FsXCase;

    ///N(case {cases}) = case { N(cases) } AND N(cocase {cases}) = case { N(cases) }
    fn focus(self, state: &mut FocusingState) -> Self::Target {
        FsXCase {
            clauses: self.clauses.focus(state),
        }
    }
}

impl<T: PrdCns> Bind for XCase<T> {
    ///bind(case {cases)[k] = ⟨μa.k(a) | case N{cases}⟩
    ///AND bind(cocase {cases)[k] = ⟨μa.k(a) | case N{cases}⟩
    fn bind(
        self,
        k: Continuation,
        state: &mut FocusingState,
    ) -> crate::syntax::statement::FsStatement {
        let new_covar = state.fresh_covar();
        let prod = crate::syntax::term::mu::FsMu::mu(&new_covar, k(new_covar.clone(), state));
        let ty = self.ty.clone();
        crate::syntax::statement::FsCut::new(prod, self.focus(state), ty).into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsXCase {
    pub clauses: Vec<FsClause>,
}

impl Print for FsXCase {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc
            .keyword(CASE)
            .append(alloc.space())
            .append(print_clauses(&self.clauses, cfg, alloc))
    }
}

impl From<FsXCase> for FsTerm {
    fn from(value: FsXCase) -> Self {
        FsTerm::XCase(value)
    }
}

impl SubstVar for FsXCase {
    type Target = FsXCase;
    fn subst_sim(self, subst: &[(Var, Var)]) -> Self::Target {
        FsXCase {
            clauses: self.clauses.subst_sim(subst),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause {
    pub xtor: Name,
    pub context: TypingContext,
    pub rhs: Rc<Statement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsClause {
    pub xtor: Name,
    pub context: FsTypingContext,
    pub case: Rc<FsStatement>,
}

impl Print for FsClause {
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
            .append(self.case.print(cfg, alloc))
            .nest(cfg.indent);
        prefix.append(tail).group()
    }
}

impl SubstVar for FsClause {
    type Target = FsClause;

    fn subst_sim(self, subst: &[(Var, Var)]) -> FsClause {
        FsClause {
            case: self.case.subst_sim(subst),
            ..self
        }
    }
}

impl Print for Clause {
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

impl FreeV for Clause {
    fn free_vars(self: &Clause) -> HashSet<Var> {
        let mut free_vars = self.rhs.free_vars();
        for bnd in &self.context.bindings {
            if let ContextBinding::VarBinding { var, ty: _ } = bnd {
                free_vars.remove(var);
            }
        }
        free_vars
    }
    fn free_covars(self: &Clause) -> HashSet<Covar> {
        let mut free_covars = self.rhs.free_covars();
        for bnd in &self.context.bindings {
            if let ContextBinding::CovarBinding { covar, ty: _ } = bnd {
                free_covars.remove(covar);
            }
        }
        free_covars
    }
}

impl UsedBinders for Clause {
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

impl Subst for Clause {
    type Target = Clause;
    fn subst_sim(
        self: &Clause,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Clause {
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

impl Uniquify for Clause {
    fn uniquify(self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> Clause {
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

impl Focusing for Clause {
    type Target = FsClause;
    ///N(K_i(x_{i,j}) => s_i ) = K_i(x_{i,j}) => N(s_i)
    fn focus(self, state: &mut FocusingState) -> FsClause {
        state.add_context(&self.context);
        FsClause {
            xtor: self.xtor,
            context: self.context.focus(state),
            case: self.rhs.focus(state),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::context::{Context, FsContextBinding};
    use crate::syntax::statement::FsCut;
    use crate::syntax::term::FsXVar;
    use crate::syntax::Chirality;
    use crate::syntax::{context::ContextBinding, statement::Cut, term::XVar, types::Ty};
    use crate::traits::Focusing;
    use std::rc::Rc;

    use super::{Clause, FsClause};

    #[test]
    fn focus_clause() {
        let result = Clause {
            xtor: "Ap".to_owned(),
            context: Context {
                bindings: vec![
                    ContextBinding::VarBinding {
                        var: "x".to_owned(),
                        ty: Ty::Int,
                    },
                    ContextBinding::CovarBinding {
                        covar: "a".to_owned(),
                        ty: Ty::Int,
                    },
                ],
            },
            rhs: Rc::new(
                Cut {
                    producer: Rc::new(XVar::var("x", Ty::Int).into()),
                    ty: Ty::Int,
                    consumer: Rc::new(XVar::covar("a", Ty::Int).into()),
                }
                .into(),
            ),
        }
        .focus(&mut Default::default());
        let expected = FsClause {
            xtor: "Ap".to_owned(),
            context: Context {
                bindings: vec![
                    FsContextBinding {
                        chi: Chirality::Prd,
                        var: "x".to_owned(),
                        ty: crate::syntax::Ty::Int,
                    },
                    FsContextBinding {
                        chi: Chirality::Cns,
                        var: "a".to_owned(),
                        ty: crate::syntax::Ty::Int,
                    },
                ],
            },
            case: Rc::new(
                FsCut {
                    producer: Rc::new(
                        FsXVar {
                            chi: Chirality::Prd,
                            var: "x".to_owned(),
                        }
                        .into(),
                    ),
                    ty: Ty::Int,
                    consumer: Rc::new(
                        FsXVar {
                            chi: Chirality::Cns,
                            var: "a".to_owned(),
                        }
                        .into(),
                    ),
                }
                .into(),
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
    };
    use std::{collections::HashSet, rc::Rc};

    fn example_cocase() -> XCase<Prd> {
        XCase {
            prdcns: Prd,
            clauses: vec![
                Clause {
                    xtor: "Fst".to_owned(),
                    context: Context {
                        bindings: vec![
                            ContextBinding::VarBinding {
                                var: "x".to_owned(),
                                ty: Ty::Int,
                            },
                            ContextBinding::CovarBinding {
                                covar: "a".to_owned(),
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
                    xtor: "Snd".to_owned(),
                    context: Context { bindings: vec![] },
                    rhs: Rc::new(
                        Cut::new(XVar::var("x", Ty::Int), XVar::covar("a", Ty::Int), Ty::Int)
                            .into(),
                    ),
                },
            ],
            ty: Ty::Decl("LPairIntInt".to_owned()),
        }
        .into()
    }

    fn example_case() -> XCase<Cns> {
        XCase {
            prdcns: Cns,
            clauses: vec![
                Clause {
                    xtor: "Nil".to_owned(),
                    context: Context { bindings: vec![] },
                    rhs: Rc::new(
                        Cut::new(XVar::var("x", Ty::Int), XVar::covar("a", Ty::Int), Ty::Int)
                            .into(),
                    ),
                },
                Clause {
                    xtor: "Cons".to_owned(),
                    context: Context {
                        bindings: vec![
                            ContextBinding::VarBinding {
                                var: "x".to_owned(),
                                ty: Ty::Int,
                            },
                            ContextBinding::VarBinding {
                                var: "xs".to_owned(),
                                ty: Ty::Decl("ListInt".to_owned()),
                            },
                            ContextBinding::CovarBinding {
                                covar: "a".to_owned(),
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
            ty: Ty::Decl("ListInt".to_owned()),
        }
        .into()
    }

    fn example_prodsubst() -> Vec<(Term<Prd>, Var)> {
        vec![(XVar::var("y", Ty::Int).into(), "x".to_owned())]
    }

    fn example_conssubst() -> Vec<(Term<Cns>, Covar)> {
        vec![(XVar::covar("b", Ty::Int).into(), "a".to_owned())]
    }

    #[test]
    fn display_cocase() {
        let result = example_cocase().print_to_string(None);
        let expected =
            "cocase { Fst(x: Int, 'a :cns Int) => <x | 'a>, Snd => <x | 'a> }".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn display_case() {
        let result = example_case().print_to_string(None);
        let expected =
            "case {\n    Nil => <x | 'a>,\n    Cons(x: Int, xs: ListInt, 'a :cns Int) => <x | 'a>\n}"
                .to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_cocase() {
        let result = example_cocase().free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars_case() {
        let result = example_case().free_vars();
        let expected = HashSet::from(["x".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_cocase() {
        let result = example_cocase().free_covars();
        let expected = HashSet::from(["a".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_covars_case() {
        let result = example_case().free_covars();
        let expected = HashSet::from(["a".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn subst_case() {
        let result = example_case().subst_sim(&example_prodsubst(), &example_conssubst());
        let expected = XCase {
            prdcns: Cns,
            clauses: vec![
                Clause {
                    xtor: "Nil".to_owned(),
                    context: Context { bindings: vec![] },
                    rhs: Rc::new(
                        Cut::new(XVar::var("y", Ty::Int), XVar::covar("b", Ty::Int), Ty::Int)
                            .into(),
                    ),
                },
                Clause {
                    xtor: "Cons".to_owned(),
                    context: Context {
                        bindings: vec![
                            ContextBinding::VarBinding {
                                var: "x".to_owned(),
                                ty: Ty::Int,
                            },
                            ContextBinding::VarBinding {
                                var: "xs".to_owned(),
                                ty: Ty::Decl("ListInt".to_owned()),
                            },
                            ContextBinding::CovarBinding {
                                covar: "a".to_owned(),
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
            ty: Ty::Decl("ListInt".to_owned()),
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
                    xtor: "Fst".to_owned(),
                    context: Context {
                        bindings: vec![
                            ContextBinding::VarBinding {
                                var: "x".to_owned(),
                                ty: Ty::Int,
                            },
                            ContextBinding::CovarBinding {
                                covar: "a".to_owned(),
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
                    xtor: "Snd".to_owned(),
                    context: Context { bindings: vec![] },
                    rhs: Rc::new(
                        Cut::new(XVar::var("y", Ty::Int), XVar::covar("b", Ty::Int), Ty::Int)
                            .into(),
                    ),
                },
            ],
            ty: Ty::Decl("LPairIntInt".to_owned()),
        };
        assert_eq!(result, expected)
    }
}
