//! Defines the [IfC]-Statement
use printer::{
    theme::ThemeExt,
    tokens::{ELSE, EQQ, GT, GTE, IF, LT, LTE, NEQ, ZERO},
    util::BracesExt,
    DocAllocator, Print,
};

use super::{ContextBinding, Covar, Statement, Var};
use crate::{
    syntax::{
        context::Chirality,
        terms::{Cns, Prd, Term},
        types::Ty,
        FsStatement,
    },
    traits::*,
};

use std::collections::{BTreeSet, HashSet};
use std::rc::Rc;

/// The comparison operator for an [IfC]-Statement
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IfSort {
    /// ==
    Equal,
    /// !=
    NotEqual,
    /// <
    Less,
    /// <=
    LessOrEqual,
    /// >
    Greater,
    /// >=
    GreaterOrEqual,
}

/// An if statement
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfC {
    /// The conditional operator
    pub sort: IfSort,
    /// The left-hand side of the comparison
    pub fst: Rc<Term<Prd>>,
    /// The right-hand side of the comparison
    /// When this is none, the left-hand side is compared to `0` (see [IfC::ifz])
    pub snd: Option<Rc<Term<Prd>>>,
    /// The then-statement
    pub thenc: Rc<Statement>,
    /// The else-statement
    pub elsec: Rc<Statement>,
}

impl IfC {
    /// Crates if with `==` comparison for given operands and then/else statements
    pub fn ife<T, U, V, W>(fst: T, snd: U, thenc: V, elsec: W) -> IfC
    where
        T: Into<Term<Prd>>,
        U: Into<Term<Prd>>,
        V: Into<Statement>,
        W: Into<Statement>,
    {
        IfC {
            sort: IfSort::Equal,
            fst: Rc::new(fst.into()),
            snd: Some(Rc::new(snd.into())),
            thenc: Rc::new(thenc.into()),
            elsec: Rc::new(elsec.into()),
        }
    }

    /// Crates if with `<` comparison for given operands and then/else statements
    pub fn ifl<T, U, V, W>(fst: T, snd: U, thenc: V, elsec: W) -> IfC
    where
        T: Into<Term<Prd>>,
        U: Into<Term<Prd>>,
        V: Into<Statement>,
        W: Into<Statement>,
    {
        IfC {
            sort: IfSort::Less,
            fst: Rc::new(fst.into()),
            snd: Some(Rc::new(snd.into())),
            thenc: Rc::new(thenc.into()),
            elsec: Rc::new(elsec.into()),
        }
    }

    /// Crates if with `== 0` comparison for give operand and then/else statements
    pub fn ifz<T, V, W>(fst: T, thenc: V, elsec: W) -> IfC
    where
        T: Into<Term<Prd>>,
        V: Into<Statement>,
        W: Into<Statement>,
    {
        IfC {
            sort: IfSort::Equal,
            fst: Rc::new(fst.into()),
            snd: None,
            thenc: Rc::new(thenc.into()),
            elsec: Rc::new(elsec.into()),
        }
    }
}

impl Typed for IfC {
    fn get_type(&self) -> Ty {
        self.thenc.get_type()
    }
}

impl Print for IfC {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let comparison = match self.sort {
            IfSort::Equal => EQQ,
            IfSort::NotEqual => NEQ,
            IfSort::Less => LT,
            IfSort::LessOrEqual => LTE,
            IfSort::Greater => GT,
            IfSort::GreaterOrEqual => GTE,
        };
        let snd = match self.snd {
            None => alloc.text(ZERO),
            Some(ref snd) => snd.print(cfg, alloc),
        };
        alloc
            .keyword(IF)
            .append(alloc.space())
            .append(self.fst.print(cfg, alloc))
            .append(alloc.space())
            .append(comparison)
            .append(alloc.space())
            .append(snd)
            .append(alloc.space())
            .append(
                alloc
                    .line()
                    .append(self.thenc.print(cfg, alloc))
                    .nest(cfg.indent)
                    .append(alloc.line())
                    .braces_anno(),
            )
            .append(alloc.space())
            .append(alloc.keyword(ELSE))
            .append(alloc.space())
            .append(
                alloc
                    .line()
                    .append(self.elsec.print(cfg, alloc))
                    .nest(cfg.indent)
                    .append(alloc.line())
                    .braces_anno(),
            )
    }
}

impl From<IfC> for Statement {
    fn from(value: IfC) -> Self {
        Statement::IfC(value)
    }
}

impl Subst for IfC {
    type Target = IfC;
    fn subst_sim(
        mut self,
        prod_subst: &[(Var, Term<Prd>)],
        cons_subst: &[(Covar, Term<Cns>)],
    ) -> Self::Target {
        self.fst = self.fst.subst_sim(prod_subst, cons_subst);
        self.snd = self.snd.subst_sim(prod_subst, cons_subst);

        self.thenc = self.thenc.subst_sim(prod_subst, cons_subst);
        self.elsec = self.elsec.subst_sim(prod_subst, cons_subst);

        self
    }
}

impl TypedFreeVars for IfC {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        self.fst.typed_free_vars(vars);
        self.snd.typed_free_vars(vars);
        self.thenc.typed_free_vars(vars);
        self.elsec.typed_free_vars(vars);
    }
}

impl Uniquify for IfC {
    fn uniquify(mut self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> IfC {
        self.fst = self.fst.uniquify(seen_vars, used_vars);
        self.snd = self.snd.uniquify(seen_vars, used_vars);

        let mut seen_vars_thenc = seen_vars.clone();
        let mut used_vars_thenc = used_vars.clone();
        self.thenc = self
            .thenc
            .uniquify(&mut seen_vars_thenc, &mut used_vars_thenc);
        self.elsec = self.elsec.uniquify(seen_vars, used_vars);
        seen_vars.extend(seen_vars_thenc);
        used_vars.extend(used_vars_thenc);

        self
    }
}

impl Focusing for IfC {
    type Target = FsStatement;
    ///N(ifc(p_1, p_2, s_1, s_2)) = bind(p_1)[λa1.bind(p_1)[λa2.ifc(a_1, a_2, N(s_1), N(s_2))]] OR
    ///N(ifz(p, s_1, s_2)) = bind(p)[λa.ifz(a, N(s_1), N(s_2))]
    fn focus(self, used_vars: &mut HashSet<Var>) -> FsStatement {
        Rc::unwrap_or_clone(self.fst).bind(
            Box::new(
                move |binding_fst: ContextBinding, used_vars: &mut HashSet<Var>| match self.snd {
                    None => FsIfC {
                        sort: self.sort,
                        fst: binding_fst.var,
                        snd: None,
                        thenc: self.thenc.focus(used_vars),
                        elsec: self.elsec.focus(used_vars),
                    }
                    .into(),
                    Some(snd) => Rc::unwrap_or_clone(snd).bind(
                        Box::new(move |binding_snd, used_vars: &mut HashSet<Var>| {
                            FsIfC {
                                sort: self.sort,
                                fst: binding_fst.var,
                                snd: Some(binding_snd.var),
                                thenc: self.thenc.focus(used_vars),
                                elsec: self.elsec.focus(used_vars),
                            }
                            .into()
                        }),
                        used_vars,
                    ),
                },
            ),
            used_vars,
        )
    }
}

/// Focused IfC
/// see [Focusing]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsIfC {
    /// The comparison operator
    pub sort: IfSort,
    /// The left-hand side of the comparison
    /// after focusing this is always a variable
    pub fst: Var,
    /// The right-hand side of the comparison
    /// after focusing this is always a variable
    /// when this is `None`, `fst` is always compared to `0`
    pub snd: Option<Var>,
    /// The then Statement
    pub thenc: Rc<FsStatement>,
    /// The else Statement
    pub elsec: Rc<FsStatement>,
}

impl Print for FsIfC {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        let comparison = match self.sort {
            IfSort::Equal => EQQ,
            IfSort::NotEqual => NEQ,
            IfSort::Less => LT,
            IfSort::LessOrEqual => LTE,
            IfSort::Greater => GT,
            IfSort::GreaterOrEqual => GTE,
        };
        let snd = match self.snd {
            None => alloc.text(ZERO),
            Some(ref snd) => snd.print(cfg, alloc),
        };
        alloc
            .keyword(IF)
            .append(alloc.space())
            .append(self.fst.print(cfg, alloc))
            .append(alloc.space())
            .append(comparison)
            .append(alloc.space())
            .append(snd)
            .append(alloc.space())
            .append(
                alloc
                    .line()
                    .append(self.thenc.print(cfg, alloc))
                    .nest(cfg.indent)
                    .append(alloc.line())
                    .braces_anno(),
            )
            .append(alloc.space())
            .append(alloc.keyword(ELSE))
            .append(alloc.space())
            .append(
                alloc
                    .line()
                    .append(self.elsec.print(cfg, alloc))
                    .nest(cfg.indent)
                    .append(alloc.line())
                    .braces_anno(),
            )
    }
}

impl From<FsIfC> for FsStatement {
    fn from(value: FsIfC) -> Self {
        FsStatement::IfC(value)
    }
}

impl SubstVar for FsIfC {
    type Target = FsIfC;
    fn subst_sim(mut self, subst: &[(Var, Var)]) -> FsIfC {
        self.fst = self.fst.subst_sim(subst);
        self.snd = self.snd.subst_sim(subst);

        self.thenc = self.thenc.subst_sim(subst);
        self.elsec = self.elsec.subst_sim(subst);

        self
    }
}

impl TypedFreeVars for FsIfC {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        vars.insert(ContextBinding {
            var: self.fst.clone(),
            chi: Chirality::Prd,
            ty: Ty::I64,
        });
        if let Some(var) = self.snd.clone() {
            vars.insert(ContextBinding {
                var,
                chi: Chirality::Prd,
                ty: Ty::I64,
            });
        }
        self.thenc.typed_free_vars(vars);
        self.elsec.typed_free_vars(vars);
    }
}

#[cfg(test)]
mod transform_tests {
    use super::{Focusing, IfSort};
    use crate::syntax::{
        statements::{Cut, Exit, FsCut, FsExit, FsIfC, IfC},
        terms::{Literal, Mu, XVar},
        types::Ty,
    };
    use std::rc::Rc;

    #[test]
    fn transform_ife1() {
        let result = IfC {
            sort: IfSort::Equal,
            fst: Rc::new(Literal::new(2).into()),
            snd: Some(Rc::new(Literal::new(1).into())),
            thenc: Rc::new(Cut::new(Literal::new(1), XVar::covar("a", Ty::I64), Ty::I64).into()),
            elsec: Rc::new(Exit::exit(XVar::var("x", Ty::I64), Ty::I64).into()),
        }
        .focus(&mut Default::default());

        let expected = FsCut::new(
            Literal::new(2),
            Mu::tilde_mu(
                "x0",
                FsCut::new(
                    Literal::new(1),
                    Mu::tilde_mu(
                        "x1",
                        FsIfC {
                            sort: IfSort::Equal,
                            fst: "x0".to_string(),
                            snd: Some("x1".to_string()),
                            thenc: Rc::new(
                                FsCut::new(Literal::new(1), XVar::covar("a", Ty::I64), Ty::I64)
                                    .into(),
                            ),
                            elsec: Rc::new(FsExit::exit("x").into()),
                        },
                        Ty::I64,
                    ),
                    Ty::I64,
                ),
                Ty::I64,
            ),
            Ty::I64,
        )
        .into();
        assert_eq!(result, expected)
    }
    #[test]
    fn transform_ife2() {
        let result = IfC {
            sort: IfSort::Equal,
            fst: Rc::new(XVar::var("x", Ty::I64).into()),
            snd: Some(Rc::new(XVar::var("x", Ty::I64).into())),
            thenc: Rc::new(Exit::exit(XVar::var("y", Ty::I64), Ty::I64).into()),
            elsec: Rc::new(
                Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64).into(),
            ),
        }
        .focus(&mut Default::default());
        let expected = FsIfC {
            sort: IfSort::Equal,
            fst: "x".to_string(),
            snd: Some("x".to_string()),
            thenc: Rc::new(FsExit::exit("y").into()),
            elsec: Rc::new(
                FsCut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64).into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_ifz1() {
        let result = IfC {
            sort: IfSort::Equal,
            fst: Rc::new(Literal::new(1).into()),
            snd: None,
            thenc: Rc::new(Cut::new(Literal::new(1), XVar::covar("a", Ty::I64), Ty::I64).into()),
            elsec: Rc::new(Exit::exit(XVar::var("x", Ty::I64), Ty::I64).into()),
        }
        .focus(&mut Default::default());
        let expected = FsCut::new(
            Literal::new(1),
            Mu::tilde_mu(
                "x0",
                FsIfC {
                    sort: IfSort::Equal,
                    fst: "x0".to_string(),
                    snd: None,
                    thenc: Rc::new(
                        FsCut::new(Literal::new(1), XVar::covar("a", Ty::I64), Ty::I64).into(),
                    ),
                    elsec: Rc::new(FsExit::exit("x").into()),
                },
                Ty::I64,
            ),
            Ty::I64,
        )
        .into();
        assert_eq!(result, expected)
    }
    #[test]
    fn transform_ifz2() {
        let result = IfC {
            sort: IfSort::Equal,
            fst: Rc::new(XVar::var("x", Ty::I64).into()),
            snd: None,
            thenc: Rc::new(Exit::exit(XVar::var("y", Ty::I64), Ty::I64).into()),
            elsec: Rc::new(
                Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64).into(),
            ),
        }
        .focus(&mut Default::default());
        let expected = FsIfC {
            sort: IfSort::Equal,
            fst: "x".to_string(),
            snd: None,
            thenc: Rc::new(FsExit::exit("y").into()),
            elsec: Rc::new(
                FsCut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64).into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
