use printer::{
    theme::ThemeExt,
    tokens::{COMMA, IFE, SEMI},
    DocAllocator, Print,
};

use super::{Covar, Statement, Var};
use crate::{
    syntax::{
        term::{Cns, Prd, Term},
        types::{Ty, Typed},
    },
    traits::{
        focus::{Bind, Focusing, FocusingState},
        free_vars::FreeV,
        substitution::Subst,
        uniquify::Uniquify,
        used_binders::UsedBinders,
    },
};

use std::{collections::HashSet, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfE {
    pub fst: Rc<Term<Prd>>,
    pub snd: Rc<Term<Prd>>,
    pub thenc: Rc<Statement>,
    pub elsec: Rc<Statement>,
}

impl Typed for IfE {
    fn get_type(&self) -> Ty {
        self.thenc.get_type()
    }
}

impl Print for IfE {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.keyword(IFE).append(
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

impl From<IfE> for Statement {
    fn from(value: IfE) -> Self {
        Statement::IfE(value)
    }
}

impl FreeV for IfE {
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

impl UsedBinders for IfE {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.fst.used_binders(used);
        self.snd.used_binders(used);
        self.thenc.used_binders(used);
        self.elsec.used_binders(used);
    }
}

impl Subst for IfE {
    type Target = IfE;
    fn subst_sim(
        &self,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Self::Target {
        IfE {
            fst: self.fst.subst_sim(prod_subst, cons_subst),
            snd: self.snd.subst_sim(prod_subst, cons_subst),
            thenc: self.thenc.subst_sim(prod_subst, cons_subst),
            elsec: self.elsec.subst_sim(prod_subst, cons_subst),
        }
    }
}

impl Uniquify for IfE {
    fn uniquify(self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> IfE {
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

        IfE {
            fst,
            snd,
            thenc,
            elsec,
        }
    }
}

impl Focusing for IfE {
    type Target = crate::syntax_var::FsStatement;
    ///N(ifz(p_1, p_2, s_1, s_2)) = bind(p_1)[λa1.bind(p_1)[λa2.ifz(a_1, a_2, N(s_1), N(s_2))]]
    fn focus(self, state: &mut FocusingState) -> crate::syntax_var::FsStatement {
        let cont = Box::new(|var_fst, state: &mut FocusingState| {
            Rc::unwrap_or_clone(self.snd).bind(
                Box::new(|var_snd: Var, state: &mut FocusingState| {
                    crate::syntax_var::statement::FsIfE {
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

#[cfg(test)]
mod transform_tests {
    use super::Focusing;
    use crate::syntax::{
        statement::{Cut, IfE},
        term::{Literal, XVar},
        types::Ty,
        Statement,
    };
    use crate::syntax_var::Chirality;
    use std::rc::Rc;

    fn example_ife1() -> IfE {
        IfE {
            fst: Rc::new(Literal::new(2).into()),
            snd: Rc::new(Literal::new(1).into()),
            thenc: Rc::new(
                Cut::new(Literal::new(1), XVar::covar("a", Ty::Int()), Ty::Int()).into(),
            ),
            elsec: Rc::new(Statement::Done(Ty::Int())),
        }
    }

    fn example_ife2() -> IfE {
        IfE {
            fst: Rc::new(XVar::var("x", Ty::Int()).into()),
            snd: Rc::new(XVar::var("x", Ty::Int()).into()),
            thenc: Rc::new(Statement::Done(Ty::Int())),
            elsec: Rc::new(
                Cut::new(
                    XVar::var("x", Ty::Int()),
                    XVar::covar("a", Ty::Int()),
                    Ty::Int(),
                )
                .into(),
            ),
        }
    }
    fn example_ife2_var() -> crate::syntax_var::statement::FsIfE {
        crate::syntax_var::statement::FsIfE {
            fst: "x".to_string(),
            snd: "x".to_string(),
            thenc: Rc::new(crate::syntax_var::FsStatement::Done()),
            elsec: Rc::new(
                crate::syntax_var::statement::FsCut::new(
                    crate::syntax::Ty::Int(),
                    crate::syntax_var::term::FsXVar::var("x"),
                    crate::syntax_var::term::FsXVar::covar("a"),
                )
                .into(),
            ),
        }
    }

    #[test]
    fn transform_ife1() {
        let result = example_ife1().focus(&mut Default::default());
        let expected = crate::syntax_var::statement::FsCut {
            ty: crate::syntax::Ty::Int(),
            producer: Rc::new(crate::syntax::term::Literal { lit: 2 }.into()),
            consumer: Rc::new(
                crate::syntax_var::term::FsMu {
                    chi: Chirality::Cns,
                    variable: "x0".to_owned(),
                    statement: Rc::new(
                        crate::syntax_var::statement::FsCut {
                            ty: crate::syntax::Ty::Int(),
                            producer: Rc::new(crate::syntax::term::Literal { lit: 1 }.into()),
                            consumer: Rc::new(
                                crate::syntax_var::term::FsMu {
                                    chi: Chirality::Cns,
                                    variable: "x1".to_owned(),
                                    statement: Rc::new(
                                        crate::syntax_var::statement::FsIfE {
                                            fst: "x0".to_string(),
                                            snd: "x1".to_string(),
                                            thenc: Rc::new(
                                                crate::syntax_var::statement::FsCut::new(
                                                    crate::syntax::Ty::Int(),
                                                    crate::syntax::term::Literal::new(1),
                                                    crate::syntax_var::term::FsXVar::covar("a"),
                                                )
                                                .into(),
                                            ),
                                            elsec: Rc::new(crate::syntax_var::FsStatement::Done()),
                                        }
                                        .into(),
                                    ),
                                }
                                .into(),
                            ),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
    #[test]
    fn transform_ife2() {
        let result = example_ife2().focus(&mut Default::default());
        let expected = example_ife2_var().into();
        assert_eq!(result, expected)
    }
}
