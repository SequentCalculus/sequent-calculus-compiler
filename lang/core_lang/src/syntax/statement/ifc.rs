use printer::{
    theme::ThemeExt,
    tokens::{COMMA, IFE, IFL, SEMI},
    DocAllocator, Print,
};

use super::{Covar, Statement, Var};
use crate::{
    syntax::{
        statement::FsStatement,
        term::{Cns, Prd, Term},
        types::Ty,
    },
    traits::*,
};

use std::{collections::HashSet, rc::Rc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IfSort {
    Equal,
    Less,
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
        let start = match self.sort {
            IfSort::Equal => alloc.keyword(IFE),
            IfSort::Less => alloc.keyword(IFL),
        };
        start.append(
            self.fst
                .print(cfg, alloc)
                .append(COMMA)
                .append(alloc.space())
                .append(self.snd.print(cfg, alloc))
                .append(SEMI)
                .append(alloc.space())
                .append(self.thenc.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(self.elsec.print(cfg, alloc))
                .parens(),
        )
    }
}

impl From<IfC> for Statement {
    fn from(value: IfC) -> Self {
        Statement::IfC(value)
    }
}

impl FreeV for IfC {
    fn free_vars(&self) -> HashSet<Var> {
        let mut free_vars = self.fst.free_vars();
        free_vars.extend(self.snd.free_vars());
        free_vars.extend(self.thenc.free_vars());
        free_vars.extend(self.elsec.free_vars());
        free_vars
    }

    fn free_covars(&self) -> HashSet<Covar> {
        let mut free_covars = self.fst.free_covars();
        free_covars.extend(self.snd.free_covars());
        free_covars.extend(self.thenc.free_covars());
        free_covars.extend(self.elsec.free_covars());
        free_covars
    }
}

impl UsedBinders for IfC {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.fst.used_binders(used);
        self.snd.used_binders(used);
        self.thenc.used_binders(used);
        self.elsec.used_binders(used);
    }
}

impl Subst for IfC {
    type Target = IfC;
    fn subst_sim(
        &self,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Self::Target {
        IfC {
            sort: self.sort,
            fst: self.fst.subst_sim(prod_subst, cons_subst),
            snd: self.snd.subst_sim(prod_subst, cons_subst),
            thenc: self.thenc.subst_sim(prod_subst, cons_subst),
            elsec: self.elsec.subst_sim(prod_subst, cons_subst),
        }
    }
}

impl Uniquify for IfC {
    fn uniquify(self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> IfC {
        let fst = self.fst.uniquify(seen_vars, used_vars);
        let snd = self.snd.uniquify(seen_vars, used_vars);
        let mut seen_vars_thenc = seen_vars.clone();
        let mut used_vars_thenc = used_vars.clone();
        let thenc = self
            .thenc
            .uniquify(&mut seen_vars_thenc, &mut used_vars_thenc);
        let elsec = self.elsec.uniquify(seen_vars, used_vars);
        seen_vars.extend(seen_vars_thenc);
        used_vars.extend(used_vars_thenc);

        IfC {
            sort: self.sort,
            fst,
            snd,
            thenc,
            elsec,
        }
    }
}

impl Focusing for IfC {
    type Target = FsStatement;
    ///N(ifz(p_1, p_2, s_1, s_2)) = bind(p_1)[λa1.bind(p_1)[λa2.ifz(a_1, a_2, N(s_1), N(s_2))]]
    fn focus(self, state: &mut FocusingState) -> FsStatement {
        let cont = Box::new(move |var_fst, state: &mut FocusingState| {
            Rc::unwrap_or_clone(self.snd).bind(
                Box::new(move |var_snd: Var, state: &mut FocusingState| {
                    FsIfC {
                        sort: self.sort,
                        fst: var_fst,
                        snd: var_snd,
                        thenc: self.thenc.focus(state),
                        elsec: self.elsec.focus(state),
                    }
                    .into()
                }),
                state,
            )
        });
        Rc::unwrap_or_clone(self.fst).bind(cont, state)
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
        let start = match self.sort {
            IfSort::Equal => alloc.keyword(IFE),
            IfSort::Less => alloc.keyword(IFL),
        };
        start.append(
            alloc
                .text(&self.fst)
                .append(COMMA)
                .append(alloc.space())
                .append(alloc.text(&self.snd))
                .append(SEMI)
                .append(alloc.space())
                .append(self.thenc.print(cfg, alloc))
                .append(COMMA)
                .append(alloc.space())
                .append(self.elsec.print(cfg, alloc))
                .parens(),
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

    fn subst_sim(self, subst: &[(Var, Var)]) -> FsIfC {
        FsIfC {
            sort: self.sort,
            fst: self.fst.subst_sim(subst),
            snd: self.snd.subst_sim(subst),
            thenc: self.thenc.subst_sim(subst),
            elsec: self.elsec.subst_sim(subst),
        }
    }
}

#[cfg(test)]
mod transform_tests {
    use super::{Focusing, IfSort};
    use crate::syntax::statement::{FsCut, FsIfC, FsStatement};
    use crate::syntax::term::Mu;
    use crate::syntax::{
        statement::{Cut, IfC},
        term::{Literal, XVar},
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
            thenc: Rc::new(Cut::new(Literal::new(1), XVar::covar("a", Ty::Int), Ty::Int).into()),
            elsec: Rc::new(Statement::Done(Ty::Int)),
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
                                FsCut::new(Literal::new(1), XVar::covar("a", Ty::Int), Ty::Int)
                                    .into(),
                            ),
                            elsec: Rc::new(FsStatement::Done()),
                        },
                        Ty::Int,
                    ),
                    Ty::Int,
                ),
                Ty::Int,
            ),
            Ty::Int,
        )
        .into();
        assert_eq!(result, expected)
    }
    #[test]
    fn transform_ife2() {
        let result = IfC {
            sort: IfSort::Equal,
            fst: Rc::new(XVar::var("x", Ty::Int).into()),
            snd: Rc::new(XVar::var("x", Ty::Int).into()),
            thenc: Rc::new(Statement::Done(Ty::Int)),
            elsec: Rc::new(
                Cut::new(XVar::var("x", Ty::Int), XVar::covar("a", Ty::Int), Ty::Int).into(),
            ),
        }
        .focus(&mut Default::default());
        let expected = FsIfC {
            sort: IfSort::Equal,
            fst: "x".to_string(),
            snd: "x".to_string(),
            thenc: Rc::new(FsStatement::Done()),
            elsec: Rc::new(
                FsCut::new(XVar::var("x", Ty::Int), XVar::covar("a", Ty::Int), Ty::Int).into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
