use printer::{
    theme::ThemeExt,
    tokens::{COMMA, IFZ, SEMI},
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
pub struct IfZ {
    pub ifc: Rc<Term<Prd>>,
    pub thenc: Rc<Statement>,
    pub elsec: Rc<Statement>,
}

impl Typed for IfZ {
    fn get_type(&self) -> Ty {
        self.thenc.get_type()
    }
}

impl Print for IfZ {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        alloc.keyword(IFZ).append(
            self.ifc
                .print(cfg, alloc)
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

impl From<IfZ> for Statement {
    fn from(value: IfZ) -> Self {
        Statement::IfZ(value)
    }
}

impl FreeV for IfZ {
    fn free_vars(&self) -> HashSet<Var> {
        let mut free_vars = self.ifc.free_vars();
        free_vars.extend(self.thenc.free_vars());
        free_vars.extend(self.elsec.free_vars());
        free_vars
    }

    fn free_covars(&self) -> HashSet<Covar> {
        let mut free_covars = self.ifc.free_covars();
        free_covars.extend(self.thenc.free_covars());
        free_covars.extend(self.elsec.free_covars());
        free_covars
    }
}

impl UsedBinders for IfZ {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.ifc.used_binders(used);
        self.thenc.used_binders(used);
        self.elsec.used_binders(used);
    }
}

impl Subst for IfZ {
    type Target = IfZ;
    fn subst_sim(
        &self,
        prod_subst: &[(Term<Prd>, Var)],
        cons_subst: &[(Term<Cns>, Covar)],
    ) -> Self::Target {
        IfZ {
            ifc: self.ifc.subst_sim(prod_subst, cons_subst),
            thenc: self.thenc.subst_sim(prod_subst, cons_subst),
            elsec: self.elsec.subst_sim(prod_subst, cons_subst),
        }
    }
}

impl Uniquify for IfZ {
    fn uniquify(self, seen_vars: &mut HashSet<Var>, used_vars: &mut HashSet<Var>) -> IfZ {
        let ifc = self.ifc.uniquify(seen_vars, used_vars);
        let mut seen_vars_thenc = seen_vars.clone();
        let mut used_vars_thenc = used_vars.clone();
        let thenc = self
            .thenc
            .uniquify(&mut seen_vars_thenc, &mut used_vars_thenc);
        let elsec = self.elsec.uniquify(seen_vars, used_vars);
        seen_vars.extend(seen_vars_thenc);
        used_vars.extend(used_vars_thenc);

        IfZ { ifc, thenc, elsec }
    }
}

impl Focusing for IfZ {
    type Target = crate::syntax_var::FsStatement;
    ///N(ifz(p, s_1, s_2)) = bind(p)[λa.ifz(a, N(s_1), N(s_2))]
    fn focus(self, state: &mut FocusingState) -> crate::syntax_var::FsStatement {
        let cont = Box::new(|var, state: &mut FocusingState| {
            crate::syntax_var::statement::FsIfZ {
                ifc: var,
                thenc: self.thenc.focus(state),
                elsec: self.elsec.focus(state),
            }
            .into()
        });

        Rc::unwrap_or_clone(self.ifc).bind(cont, state)
    }
}

#[cfg(test)]
mod transform_tests {
    use super::Focusing;
    use crate::syntax::{
        statement::{Cut, IfZ},
        term::{Literal, XVar},
        types::Ty,
        Statement,
    };
    use crate::syntax_var::Chirality;
    use std::rc::Rc;

    fn example_ifz1() -> IfZ {
        IfZ {
            ifc: Rc::new(Literal::new(1).into()),
            thenc: Rc::new(
                Cut::new(Literal::new(1), XVar::covar("a", Ty::Int()), Ty::Int()).into(),
            ),
            elsec: Rc::new(Statement::Done(Ty::Int())),
        }
    }

    fn example_ifz2() -> IfZ {
        IfZ {
            ifc: Rc::new(XVar::var("x", Ty::Int()).into()),
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
    fn example_ifz2_var() -> crate::syntax_var::statement::FsIfZ {
        crate::syntax_var::statement::FsIfZ {
            ifc: "x".to_string(),
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
    fn transform_ifz1() {
        let result = example_ifz1().focus(&mut Default::default());
        let expected = crate::syntax_var::statement::FsCut {
            ty: crate::syntax::Ty::Int(),
            producer: Rc::new(crate::syntax::term::Literal { lit: 1 }.into()),
            consumer: Rc::new(
                crate::syntax::term::mu::FsMu {
                    chi: Chirality::Cns,
                    variable: "x0".to_owned(),
                    statement: Rc::new(
                        crate::syntax_var::statement::FsIfZ {
                            ifc: "x0".to_string(),
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
        .into();
        assert_eq!(result, expected)
    }
    #[test]
    fn transform_ifz2() {
        let result = example_ifz2().focus(&mut Default::default());
        let expected = example_ifz2_var().into();
        assert_eq!(result, expected)
    }
}
