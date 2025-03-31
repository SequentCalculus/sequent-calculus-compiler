use printer::{
    theme::ThemeExt,
    tokens::{ELSE, EQQ, IF, LT, LTE, NEQ},
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IfSort {
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfC {
    pub sort: IfSort,
    pub fst: Rc<Term<Prd>>,
    pub snd: Rc<Term<Prd>>,
    pub thenc: Rc<Statement>,
    pub elsec: Rc<Statement>,
}

impl IfC {
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
            snd: Rc::new(snd.into()),
            thenc: Rc::new(thenc.into()),
            elsec: Rc::new(elsec.into()),
        }
    }

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
            snd: Rc::new(snd.into()),
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
        };
        alloc
            .keyword(IF)
            .append(alloc.space())
            .append(self.fst.print(cfg, alloc))
            .append(alloc.space())
            .append(comparison)
            .append(alloc.space())
            .append(self.snd.print(cfg, alloc))
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
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>, state: &TypedFreeVarsState) {
        self.fst.typed_free_vars(vars, state);
        self.snd.typed_free_vars(vars, state);
        self.thenc.typed_free_vars(vars, state);
        self.elsec.typed_free_vars(vars, state);
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
    ///N(ifc(p_1, p_2, s_1, s_2)) = bind(p_1)[λa1.bind(p_1)[λa2.ifz(a_1, a_2, N(s_1), N(s_2))]]
    fn focus(self, used_vars: &mut HashSet<Var>) -> FsStatement {
        let cont = Box::new(move |var_fst, used_vars: &mut HashSet<Var>| {
            Rc::unwrap_or_clone(self.snd).bind(
                Box::new(move |var_snd: Var, used_vars: &mut HashSet<Var>| {
                    FsIfC {
                        sort: self.sort,
                        fst: var_fst,
                        snd: var_snd,
                        thenc: self.thenc.focus(used_vars),
                        elsec: self.elsec.focus(used_vars),
                    }
                    .into()
                }),
                used_vars,
            )
        });
        Rc::unwrap_or_clone(self.fst).bind(cont, used_vars)
    }
}

/// Focused IfC
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsIfC {
    pub sort: IfSort,
    pub fst: Var,
    pub snd: Var,
    pub thenc: Rc<FsStatement>,
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
        };
        alloc
            .keyword(IF)
            .append(alloc.space())
            .append(self.fst.print(cfg, alloc))
            .append(alloc.space())
            .append(comparison)
            .append(alloc.space())
            .append(self.snd.print(cfg, alloc))
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
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>, state: &TypedFreeVarsState) {
        self.thenc.typed_free_vars(vars, state);
        self.elsec.typed_free_vars(vars, state);
        vars.insert(ContextBinding {
            var: self.fst.clone(),
            chi: Chirality::Prd,
            ty: Ty::I64,
        });
        vars.insert(ContextBinding {
            var: self.snd.clone(),
            chi: Chirality::Prd,
            ty: Ty::I64,
        });
    }
}

#[cfg(test)]
mod transform_tests {
    use super::{Focusing, IfSort};
    use crate::syntax::FsStatement;
    use crate::syntax::{
        statements::{Cut, FsCut, FsIfC, IfC},
        terms::{Literal, Mu, XVar},
        types::Ty,
        Statement,
    };
    use std::rc::Rc;

    #[test]
    fn transform_ife1() {
        let result = IfC {
            sort: IfSort::Equal,
            fst: Rc::new(Literal::new(2).into()),
            snd: Rc::new(Literal::new(1).into()),
            thenc: Rc::new(Cut::new(Literal::new(1), XVar::covar("a", Ty::I64), Ty::I64).into()),
            elsec: Rc::new(Statement::Done(Ty::I64)),
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
                            snd: "x1".to_string(),
                            thenc: Rc::new(
                                FsCut::new(Literal::new(1), XVar::covar("a", Ty::I64), Ty::I64)
                                    .into(),
                            ),
                            elsec: Rc::new(FsStatement::Done()),
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
            snd: Rc::new(XVar::var("x", Ty::I64).into()),
            thenc: Rc::new(Statement::Done(Ty::I64)),
            elsec: Rc::new(
                Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64).into(),
            ),
        }
        .focus(&mut Default::default());
        let expected = FsIfC {
            sort: IfSort::Equal,
            fst: "x".to_string(),
            snd: "x".to_string(),
            thenc: Rc::new(FsStatement::Done()),
            elsec: Rc::new(
                FsCut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64).into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
